pub type GenerativeFunction = fn(f32) -> f32;
type Function = fn(&[f32]) -> f32;

fn function_from_string(s : &str) -> GenerativeFunction{
    match s {
        "sin" => f32::sin,
        "cos" => f32::cos,
        "tan" => f32::tan,
        "cot" => |x| 1f32 / f32::tan(x),
        "sec" => |x| 1f32 / f32::cos(x),
        "csc" => |x| 1f32 / f32::sin(x),
        "abs" => f32::abs,
        "ceil" => f32::ceil,
        "floor" => f32::floor,
        "round" => f32::round,
        "exp" => f32::exp,
        "ln" => f32::ln,
        "log" => f32::log10,
        "sqrt" => f32::sqrt,
        "asin" => f32::asin,
        "acos" => f32::acos,
        "atan" => f32::atan,
        "sinh" => f32::sinh,
        "cosh" => f32::cosh,
        "tanh" => f32::tanh,
        "asinh" => f32::asinh,
        "acosh" => f32::acosh,
        "atanh" => f32::atanh,
        _ => |x| x
    }
}

struct Equation{
    params : Vec<Equation>,
    function : Function
}

impl Equation{
    fn getFunction(&self) -> GenerativeFunction{
        let param_values : Vec<f32> = self.params.iter().map(|e| e.getFunction()(x)).collect();
        return |x| -> f32 {
            (self.function)(&param_values)  
        }
    }
}