use core::f32;
use std::{vec};
use std::collections::HashMap;

pub type SimpleFunction = fn(f32) -> f32;
type MultiFunction = fn(&[f32]) -> f32;
enum Function {
    MultiFunction(MultiFunction),
    SimpleFunction(SimpleFunction),
    Constant (f32),
    Iterator(Operator),
    InputX,
    Variable(String),
    Assign,
    If
}

fn function_from_string(s : &str) -> Option<Function>{

    match s.to_ascii_lowercase().as_str() {
        "sin" => Some(Function::SimpleFunction(f32::sin)),
        "cos" => Some(Function::SimpleFunction(f32::cos)),
        "tan" => Some(Function::SimpleFunction(f32::tan)),
        "cot" => Some(Function::SimpleFunction(|x| 1f32 / f32::tan(x))),
        "sec" => Some(Function::SimpleFunction(|x| 1f32 / f32::cos(x))),
        "csc" => Some(Function::SimpleFunction(|x| 1f32 / f32::sin(x))),
        "abs" => Some(Function::SimpleFunction(f32::abs)),
        "ceil" => Some(Function::SimpleFunction(f32::ceil)),
        "floor" => Some(Function::SimpleFunction(f32::floor)),
        "round" => Some(Function::SimpleFunction(f32::round)),
        "exp" => Some(Function::SimpleFunction(f32::exp)),
        "ln" => Some(Function::SimpleFunction(f32::ln)),
        "log" => Some(Function::SimpleFunction(f32::log10)),
        "sqrt" => Some(Function::SimpleFunction(f32::sqrt)),
        "asin" => Some(Function::SimpleFunction(f32::asin)),
        "acos" => Some(Function::SimpleFunction(f32::acos)),
        "atan" => Some(Function::SimpleFunction(f32::atan)),
        "sinh" => Some(Function::SimpleFunction(f32::sinh)),
        "cosh" => Some(Function::SimpleFunction(f32::cosh)),
        "tanh" => Some(Function::SimpleFunction(f32::tanh)),
        "asinh" => Some(Function::SimpleFunction(f32::asinh)),
        "acosh" => Some(Function::SimpleFunction(f32::acosh)),
        "atanh" => Some(Function::SimpleFunction(f32::atanh)),

        "pi" => Some(Function::Constant(f32::consts::PI)),
        "e" => Some(Function::Constant(f32::consts::E)),

        "max" => Some(Function::MultiFunction(|x| x.iter().fold(f32::MIN, |max, next| max.max(*next)))),
        "min" => Some(Function::MultiFunction(|x| x.iter().fold(f32::MAX, |min, next| min.min(*next)))),
        "sum" => Some(Function::Iterator(Operator::Add)),
        "prod" => Some(Function::Iterator(Operator::Mul)),
        "if" => Some(Function::If),
        
        "pow" => Some(Function::MultiFunction(|x| x[0].powf(x[1]))),
        _ => None
    }
}

pub struct Expression{
    params : Vec<Expression>,
    function : Function
}

impl Expression{

    pub fn evaluate(&self, x : f32, variables : &mut HashMap<String, f32>) -> f32{
        let param_values : Vec<f32> = match &self.function {
            Function::Iterator(_) | Function::If => vec![],
            _ => self.params.iter().map(|e| e.evaluate(x, variables)).collect()
        };
        match &self.function {
            Function::SimpleFunction(f) => {
                f(param_values[0])
            },
            Function::MultiFunction(f) => {
                f(&param_values)
            },
            Function::Constant(c) => {
                *c
            }
            Function::Variable(s) => {
                match variables.get(s) {
                    Some(res) => *res,
                    None => 0.0
                }
            }
            Function::InputX => {
                x
            },
            Function::If => {
                if self.params.len() < 3 { return f32::NAN; }
                // Lazy evaluation
                if self.params[0].evaluate(x, variables) > 0.0 { self.params[1].evaluate(x, variables) }
                else{ self.params[2].evaluate(x, variables) }
            },
            Function::Assign =>{
                match &self.params[0].function {
                    Function::Variable(s) => {variables.insert(s.to_string(), param_values[1]); },
                    _ => {return f32::NAN;}
                }
                param_values[1]
            },
            Function::Iterator(o) =>{
                match &self.params[0].function {
                    Function::Assign => {
                        match &self.params[0].params[0].function {
                            Function::Variable(it) => {
                                let mut intermediate_variables = variables.clone();

                                let mut res = match o {
                                    Operator::Add => 0.0,
                                    Operator::Mul => 1.0,
                                    _ => panic!("Internal error non authorised operation in iterator")
                                };

                                let max = self.params[1].evaluate(x, variables).round() as i32;
                                let mut current = self.params[0].params[1].evaluate(x, variables).round() as i32;
                                
                                while current <= max {
                                    intermediate_variables.insert(it.to_string(), current as f32);

                                    res = match o {
                                        Operator::Add => res + self.params[2].evaluate(x, &mut intermediate_variables),
                                        Operator::Mul => res * self.params[2].evaluate(x, &mut intermediate_variables),
                                        _ => panic!("Internal error non authorised operation in iterator")
                                    };

                                    current = current + 1;
                                }
                                res
                            },  
                            _ => {return f32::NAN;}
                        }
                    }
                    _ => {return f32::NAN;} 
                }
            }
        }
    }

    pub fn simple_evaluate(&self, x : f32) -> f32 {
        let mut variables : HashMap<String,f32> = HashMap::new();
        self.evaluate(x, &mut variables)
    }

    pub fn create_from_function(f : SimpleFunction) -> Expression {
        Expression {
            params: vec![],
            function: Function::SimpleFunction(f),
        }
    }
}


enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    ParensOpen,
    ParensClose,
    Space,
    Comma,
    Factorial,
    Square,
    Assign,
    TestEQU,
    TestNEQ,
    TestLEQ,
    TestLSS,
    TestGEQ,
    TestGTR,
}

impl Operator {
    fn from_char(c: char, next: Option<&char>) -> Option<(Self, usize)> {
        match c {
            '+' => Some((Operator::Add, 1)),
            '-' => Some((Operator::Sub, 1)),
            '*' => Some((Operator::Mul, 1)),
            '/' => Some((Operator::Div, 1)),
            '^' => Some((Operator::Pow, 1)),
            '(' => Some((Operator::ParensOpen, 1)),
            ')' => Some((Operator::ParensClose, 1)),
            ' ' => Some((Operator::Space, 1)),
            ',' => Some((Operator::Comma, 1)),
            '²' => Some((Operator::Square, 1)),
            '=' => {match next {
                    Some('=') =>Some((Operator::TestEQU, 2)),
                    _ => Some((Operator::Assign, 1)),
            }},
            '!' => {match next {
                    Some('=') =>Some((Operator::TestNEQ, 2)),
                    _ => Some((Operator::Factorial, 1)),
            }}, 
            '<' => {match next {
                    Some('=') =>Some((Operator::TestLEQ, 2)),
                    _ => Some((Operator::TestLSS, 1)),
            }},
            '>' => {match next {
                    Some('=') =>Some((Operator::TestGEQ, 2)),
                    _ => Some((Operator::TestGTR, 1)),
            }},

            _ => None,
        }
    }
    fn to_string(&self) -> &str {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Pow => "^",
            Operator::ParensOpen => "(",
            Operator::ParensClose => ")",
            Operator::Space => " ",
            Operator::Comma => ",",
            Operator::Factorial => "!",
            Operator::Square => "²",
            Operator::Assign => "=",
            Operator::TestEQU => "==",
            Operator::TestNEQ => "!=",
            Operator::TestLEQ => "<=",
            Operator::TestLSS => "<",
            Operator::TestGEQ => ">=",
            Operator::TestGTR => ">",
            }
    }

    fn precedence(&self) -> u8 {
        match self {
            Operator::Add => 2,
            Operator::Sub => 2,
            Operator::Mul => 3,
            Operator::Div => 3,
            Operator::Pow => 4,
            Operator::Square => 5, 
            Operator::ParensOpen => 255,
            Operator::ParensClose => 255,
            Operator::Space => 0,
            Operator::Comma => 255,
            Operator::Factorial => 5,
            Operator::Assign => 0,
            Operator::TestEQU => 1,
            Operator::TestNEQ => 1,
            Operator::TestLEQ => 1,
            Operator::TestLSS => 1,
            Operator::TestGEQ => 1,
            Operator::TestGTR => 1,
        }
    }

    fn get_function(&self) -> Function{
        match self {
            Operator::Add => Function::MultiFunction(|x| x[0] + x[1]),
            Operator::Sub => Function::MultiFunction(|x| x[0] - x[1]),
            Operator::Mul | Operator::Space => Function::MultiFunction(|x| x[0] * x[1]),
            Operator::Div => Function::MultiFunction(|x| x[0] / x[1]),
            Operator::Pow => Function::MultiFunction(|x| x[0].powf(x[1])),
            Operator::Square => Function::SimpleFunction(|x| x * x),
            Operator::Assign => Function::Assign,
            Operator::Factorial => Function::SimpleFunction(|x| {
                if x < 0.0 {
                    return f32::NAN;
                }
                let mut result = 1.0;
                let mut n = x as u32;
                while n > 1 {
                    result *= n as f32;
                    n -= 1;
                }
                result
            }),
            Operator::TestEQU => Function::MultiFunction(|x| if x[0] == x[1] {1.0} else {0.0}),
            Operator::TestNEQ => Function::MultiFunction(|x| if x[0] != x[1] {1.0} else {0.0}),
            Operator::TestLEQ => Function::MultiFunction(|x| if x[0] <= x[1] {1.0} else {0.0}),
            Operator::TestLSS => Function::MultiFunction(|x| if x[0] <  x[1] {1.0} else {0.0}),
            Operator::TestGEQ => Function::MultiFunction(|x| if x[0] >= x[1] {1.0} else {0.0}),
            Operator::TestGTR => Function::MultiFunction(|x| if x[0] >  x[1] {1.0} else {0.0}),
            _ => panic!("No function for this operator"),
        }
    }
}

enum Token {
    Operator(Operator),
    Identifier(String),
}

fn split_operator(s : &str) -> Vec<Token>{
    let mut parts : Vec<Token> = vec![];
    let mut current_expr = String::new();
    let mut is_numeric = true;
    let mut index  = 0;
    let vec_chars: Vec<char> = s.chars().collect();

    while index < vec_chars.len() {
        let c = vec_chars[index];
        match Operator::from_char(c, vec_chars.get(index+1)) {
            Some((op, nb_char)) => {
                if !current_expr.is_empty() {
                    match parts.last() {
                        Some(Token::Operator(Operator::ParensClose)) => parts.push(Token::Operator(Operator::Mul)),
                        _ => ()
                    }

                    parts.push(Token::Identifier(current_expr));
                }

                match op {
                    Operator::ParensOpen => {
                        if !parts.is_empty(){
                            if let Token::Identifier(last_id) = parts.last().unwrap() {
                                if matches!(function_from_string(last_id), None) {
                                    parts.push(Token::Operator(Operator::Mul));
                                }
                            }
                        }
                    },
                    _ => ()
                }
                parts.push(Token::Operator(op));
                current_expr = String::new();
                index += nb_char;
            }
            None => {
                if !(c.is_ascii_digit() || c == '.') {
                    if !current_expr.is_empty() && is_numeric {
                        parts.push(Token::Identifier(current_expr));
                        parts.push(Token::Operator(Operator::Mul));
                        current_expr = String::new();
                    }
                    is_numeric = false;
                }
                current_expr.push(c);
                index += 1;
            }
        }

        
    }

    if !current_expr.is_empty() {
        match parts.last() {
            Some(Token::Operator(Operator::ParensClose)) => parts.push(Token::Operator(Operator::Mul)),
            _ => ()
        }
        parts.push(Token::Identifier(current_expr));
    }

    return parts;
}

// Shunting yard algorithm
fn filter_tokens_priority(input : &mut Vec<Token>) -> Result<Vec<Token>, String>{
    let mut out = Vec::with_capacity(input.len());
    let mut operator_queue : Vec<Token> = Vec::new();

    while !input.is_empty() {
        match &input[0] {
            Token::Operator(Operator::ParensOpen) => {
                operator_queue.push(input.remove(0));
            },
            Token::Operator(Operator::ParensClose) => {
                while !operator_queue.is_empty() {
                    match &operator_queue.last().unwrap(){
                        Token::Operator(Operator::ParensOpen)=> {break;}
                        Token::Operator(_) => {
                                out.push(operator_queue.pop().unwrap());
                        }
                        Token::Identifier(_) => panic!("Internal error"),
                    }
                }
                if operator_queue.is_empty() { return Err("Unbalanced ')'".to_string())}
                operator_queue.pop();

                input.remove(0);

                match &operator_queue.last() {
                    Some(Token::Identifier(_)) => {out.push(Token::Operator(Operator::ParensOpen)); out.push(operator_queue.pop().unwrap());},
                    _ => ()
                }
            },
            Token::Operator(Operator::Comma) => {
                while !operator_queue.is_empty() {
                    match &operator_queue.last().unwrap(){
                        Token::Operator(Operator::ParensOpen)=> {break;}
                        Token::Operator(_) => {
                                out.push(operator_queue.pop().unwrap());
                        }
                        Token::Identifier(_) => panic!("Internal error"),
                    }
                }
                out.push(input.remove(0))
            }
            Token::Operator(Operator::Space) => {
                input.remove(0); // Drop it
            },
            Token::Operator(o1) => {
                while !operator_queue.is_empty() {
                    match &operator_queue.last().unwrap(){
                        Token::Operator(Operator::ParensOpen | Operator::Comma)=> {break;}
                        Token::Operator(o2) => {
                            if o2.precedence() >= o1.precedence() {
                                out.push(operator_queue.pop().unwrap());
                            }else{
                                break;
                            }
                        }
                        Token::Identifier(_) => panic!("Internal error"),
                    }
                }
                operator_queue.push(input.remove(0));
            },
            Token::Identifier(_) => {
                match input.get(1) {
                    Some(Token::Operator(Operator::ParensOpen)) => {
                        // Function add id both to operator queue
                        operator_queue.push(input.remove(0));
                        operator_queue.push(input.remove(0));
                    }
                    _ => out.push(input.remove(0))
                }
            },
        }
    }
    while !operator_queue.is_empty() {
        match operator_queue.last().unwrap(){
            Token::Operator(Operator::ParensOpen) => {return Err("Unbalanced '('".to_string());},
            Token::Operator(Operator::Comma) => {return Err("Unexpected ','".to_string());},
            _ => ()
        }
        out.push(operator_queue.pop().unwrap());
    }
    
    out.reverse();
    Ok(out)
}

fn match_expression(tokens : &mut Vec<Token>) -> Result<Expression, String>{
    if tokens.is_empty() {return Err("Expression was expected but none found".to_string()); }
    match tokens.remove(0) {
        Token::Identifier(s) => { 
            let func = match function_from_string(&s) {
                None => {
                    if s == "x" { Function::InputX
                    }else{ 
                        match s.parse() {
                            Ok(res) => Function::Constant(res),
                            Err(_) => Function::Variable(s.clone())
                        } 
                    }
                },
                Some(function) => {function}
            };
            match &tokens.get(0) {
                Some(Token::Operator(Operator::ParensOpen)) => {
                    tokens.remove(0);
                    match func {
                        Function::Assign | Function::Constant(_) | Function::Variable(_) | Function::InputX => Ok(Expression{function: func, params: vec![]}),
                        Function::SimpleFunction(_) => Ok(Expression { function: func, params: vec![match_expression(tokens)?] }),
                        Function::MultiFunction(_) | Function::Iterator(_) | Function::If => {
                            let mut params : Vec<Expression> = vec![match_expression(tokens)?];
                            loop{
                                match tokens.get(0) {
                                    Some(Token::Operator(Operator::Comma)) => {tokens.remove(0); params.insert(0, match_expression(tokens)?);},
                                    _ => break,
                                };
                            }
                            Ok(Expression { function: func, params : params})
                        }
                    }
                }
                _ => { 
                    match func {
                        Function::MultiFunction(_) | Function::SimpleFunction(_) | Function::Iterator(_) | Function::If => Err(format!("Error unexpected identifier : {}", s)),
                        Function::Assign |Function::Constant(_) | Function::Variable(_) | Function::InputX => Ok(Expression{function : func, params: vec![]}),
                    }
                }
            }
        },
        Token::Operator(o @ (Operator::Factorial | Operator::Square)) => {
            Ok(Expression{function : o.get_function(), params: vec![match_expression(tokens)?]})
        },
        Token::Operator(Operator::Sub) => { 
            // Switch between unary op and binary op
            let mut params = vec![match_expression(tokens)?];
            params.insert(0, match &tokens.get(0) {
                None => Expression { params: vec![], function: Function::Constant(0.0) },
                _ => match_expression(tokens)?,
            });
            Ok(Expression{function: Operator::Sub.get_function(), params: params})
        },
        Token::Operator(Operator::ParensOpen) => { 
            panic!("Internal error, Parenthesis should have been removed");   
        },
        Token::Operator(Operator::Space) => {
            panic!("Internal error, Spaces should have been removed");
        },
        Token::Operator(o @ (Operator::Comma | Operator::ParensClose)) => {
            Err(format!("Unwaited operator : {}", o.to_string()))
        }
        Token::Operator(o) =>{
            let e2 = match_expression(tokens)?;
            Ok(Expression{function: o.get_function(), params: vec![match_expression(tokens)?, e2]})
        }
    }

}

pub fn parse_expression(s : &str) -> Result<Expression, String>{
    let mut tokens = split_operator(s);
    let mut filtered = filter_tokens_priority(&mut tokens)?;
    Ok(match_expression(&mut filtered)?)
}

pub fn test_filter(s: String, variables : &mut HashMap<String, f32>) {
    let mut tokens = split_operator(&s);
    let res_parsing = filter_tokens_priority(&mut tokens);
    match res_parsing {
        Err(e) => print!("Parsing failed with : {}", e),
        Ok(new_tokens) =>{
            let mut res : String = String::new();
            for t in new_tokens.iter() {
                match t {
                    Token::Operator(operator) => res += &operator.to_string(),
                    Token::Identifier(s) => res += &s,
                }
            }
            println!("Pased           : {}", res);
            println!("Evaluated (30.0): {}", match parse_expression(&s){
                Ok(expression) => expression.evaluate(30.0, variables).to_string(),
                Err(e) => e
            })
        }
    }

}