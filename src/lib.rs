mod equation;
use std::sync::{OnceLock, Mutex};

use std::os::raw::{c_char, c_int};
use std::ptr;

use crate::equation::Expression;

struct AppState {
    variables: std::collections::HashMap<String, f32>,
    expressions: Vec<Expression>,
    last_error: Option<String>
}

static STATE: OnceLock<Mutex<AppState>> = OnceLock::new();

fn get_state() -> &'static Mutex<AppState> {
    STATE.get_or_init(|| Mutex::new(AppState {
        variables: Default::default(),
        expressions: vec![],
        last_error: None
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn parse(expression: *const std::os::raw::c_char) -> i32 {
    use std::ffi::CStr;
    let input = unsafe { CStr::from_ptr(expression) }.to_string_lossy().into_owned();
    let mut state = get_state().lock().unwrap();
    match equation::parse_expression(&input) {
        Ok(res) => {state.expressions.push(res); state.last_error = None; (state.expressions.len()-1) as i32},
        Err(e) => {state.last_error = Some(e); -1}
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn evaluate(in_index: i32, x: f32) -> f32 {
    let index = in_index as usize;
    let mut state = get_state().lock().unwrap();

    if matches!(state.expressions.get(index), Some(_)){
        // need to remove it from the state to un borrow it
        let droped = state.expressions.remove(index);
        let res = droped.evaluate(x, &mut state.variables);
        state.expressions.insert(index, droped);
        state.last_error = None;
        res
    }else{
        state.last_error = Some("Index of evaluated expression not found".to_string()); 0.0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_last_error_buff(buf: *mut c_char, buf_len: c_int) -> c_int {
    if buf.is_null() || buf_len <= 0 {
        return -1;
    }

    let state = get_state().lock().unwrap();
    let msg = state.last_error.as_deref().unwrap_or("");

    // Copy up to buf_len - 1 bytes
    let bytes = msg.as_bytes();
    let len = bytes.len().min((buf_len - 1) as usize);

    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), buf as *mut u8, len);
        *buf.add(len) = 0; // null terminator
    }

    len as c_int // return actual written length
}

#[unsafe(no_mangle)]
pub extern "C" fn get_last_error() -> *const std::os::raw::c_char {
    let state = get_state().lock().unwrap();
    let msg = state.last_error.clone().unwrap_or_else(|| "".to_string());
    std::ffi::CString::new(msg).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn free_last_error(ptr: *mut std::os::raw::c_char) {
    if !ptr.is_null() {
        unsafe { drop(std::ffi::CString::from_raw(ptr)) }
    }
}