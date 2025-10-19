use core::panic;
use std::vec;

pub type SimpleFunction = fn(f32) -> f32;
type MultiFunction = fn(&[f32]) -> f32;
enum Function {
    MultiFunction(MultiFunction),
    SimpleFunction(SimpleFunction),
    Constant (f32),
    Variable
}

fn function_from_string(s : &str) -> Option<Function>{
    match s {
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
        
        "pow" => Some(Function::MultiFunction(|x| x[0].powf(x[1]))),
        _ => None
    }
}

pub struct Expression{
    params : Vec<Expression>,
    function : Function
}

impl Expression{
    pub fn evaluate(&self, x : f32) -> f32{
        let param_values : Vec<f32> = self.params.iter().map(|e| e.evaluate(x)).collect();
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
            Function::Variable => {
                x
            }
        }
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
}

impl Operator {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Sub),
            '*' => Some(Operator::Mul),
            '/' => Some(Operator::Div),
            '^' => Some(Operator::Pow),
            '(' => Some(Operator::ParensOpen),
            ')' => Some(Operator::ParensClose),
            ' ' => Some(Operator::Space),
            ',' => Some(Operator::Comma),
            '!' => Some(Operator::Factorial),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Operator::Add => '+',
            Operator::Sub => '-',
            Operator::Mul => '*',
            Operator::Div => '/',
            Operator::Pow => '^',
            Operator::ParensOpen => '(',
            Operator::ParensClose => ')',
            Operator::Space => ' ',
            Operator::Comma => ',',
            Operator::Factorial => '!',
        }
    }

    fn get_function(&self) -> Function{
        match self {
            Operator::Add => Function::MultiFunction(|x| x[0] + x[1]),
            Operator::Sub => Function::MultiFunction(|x| x[0] - x[1]),
            Operator::Mul | Operator::Space => Function::MultiFunction(|x| x[0] * x[1]),
            Operator::Div => Function::MultiFunction(|x| x[0] / x[1]),
            Operator::Pow => Function::MultiFunction(|x| x[0].powf(x[1])),
            Operator::Factorial => Function::SimpleFunction(|x| {
                if x < 0.0 {
                    panic!("Factorial is not defined for negative numbers");
                }
                let mut result = 1.0;
                let mut n = x as u32;
                while n > 1 {
                    result *= n as f32;
                    n -= 1;
                }
                result
            }),
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
    for c in s.chars() {
        match Operator::from_char(c) {
            Some(op) => {
                if !current_expr.is_empty() {
                    parts.push(Token::Identifier(current_expr));
                }
                parts.push(Token::Operator(op));
                current_expr = String::new();
            }
            None => {
                current_expr.push(c);
            }
        }
    }

    if !current_expr.is_empty() {
        parts.push(Token::Identifier(current_expr));
    }

    return parts;
}

pub struct ExpressionParsed{
    params : Vec<ExpressionParsed>,
    token : Option<Token>
}


fn match_id(tokens : &mut Vec<Token>) -> ExpressionParsed{
    let mut tree = ExpressionParsed{token: None, params: vec![]};
    let mut last_tokens : Vec<Token> = vec![];
    loop{
        match last_tokens.last() {
        None => {
            if tokens.is_empty() {
                return tree; // Empty ID
            }
            match &tokens[0] {
                Token::Identifier(s) => { 
                    last_tokens.push(Token::Identifier(s.clone()));
                    tree.token = Some(tokens.remove(0));
                }
                Token::Operator(Operator::ParensOpen) => { 
                    last_tokens.push(tokens.remove(0));
                    tree.params.push(match_expressions(tokens));    
                }
                Token::Operator(Operator::Space) => {
                        tokens.remove(0);
                }
                _ => {return tree;} // Empty ID
            }
        }
        Some(Token::Identifier(_)) => {
            if tokens.is_empty() {
                return tree; // Classic ID + EOS
            }
            match &tokens[0] {
                Token::Operator(Operator::ParensOpen) => { 
                    last_tokens.push(tokens.remove(0));
                    tree.params.push(match_expressions(tokens));    
                }
                Token::Operator(Operator::Space) => {
                        tokens.remove(0);
                }
                _ => {return tree;} // Wrong token -> start of the new one
            }
        }
        Some(Token::Operator(Operator::ParensOpen) | Token::Operator(Operator::Comma)) => {
            if tokens.is_empty() {
                panic!("Unwaited end of expression")
            }
            match &tokens[0] {
                Token::Operator(Operator::ParensClose) => { 
                    tokens.remove(0);
                    return tree; // End of function / parenthesis
                }
                Token::Operator(Operator::Space) => {
                        tokens.remove(0);
                }
                Token::Operator(Operator::Comma) => { 
                    if last_tokens.iter().find(|token| match token {Token::Operator(Operator::ParensOpen) => true, _ => false}).is_none() {
                        panic!("Unwaited operator : ,")
                    }
                    last_tokens.push(tokens.remove(0));
                    tree.params.push(match_expressions(tokens));   
                }
                
                Token::Identifier(s) => {panic!("Unwaited identifier : {}", s)} 
                Token::Operator(o) => {panic!("Unwaited operator : {}", o.to_char())} 
            }
        }
        _ => panic!("Internal error")

        }
    }
}

fn match_expressions(tokens : &mut Vec<Token>) -> ExpressionParsed{
    let mut tree = match_id(tokens);
    loop{
        if tokens.is_empty() {
            return tree;
        }else{
            match &tokens[0] {
                Token::Identifier(_) => {
                    return tree;
                },
                Token::Operator(o) => match o {
                    Operator::Add | Operator::Sub | Operator::Mul | Operator::Div | Operator::Pow => {
                        tree = ExpressionParsed{token: Some(tokens.remove(0)), params: vec![tree, match_id(tokens)]};
                    }
                    Operator::Factorial => {
                        return ExpressionParsed{token: Some(tokens.remove(0)), params: vec![tree]};
                    }
                    Operator::Space => {
                        tokens.remove(0);
                    }
                    _ => { return tree; }
                }
            }
        }
    }
}


fn evaluate_parsed_expression(e : &ExpressionParsed) -> Expression{
    let func = match &e.token {
        Some(Token::Identifier(s)) => {
            if s.is_empty() {
                Function::SimpleFunction(|x| x)
            }else{
                match function_from_string(s) {
                    None => {
                        //TODO parse variables
                        if s == "x" {
                            Function::Variable
                        }else{
                            let value : f32 = s.parse().expect("Failed to parse number");
                            Function::Constant(value)
                        }
                    },
                    Some(function) => {function}
                }
            }
        },
        Some(Token::Operator(o)) => {
            o.get_function()
        },
        None => {
            Function::SimpleFunction(|x| x)
        }
    };
    let params : Vec<Expression> = e.params.iter().map(|p| evaluate_parsed_expression(p)).collect();
    Expression{
        params,
        function: func
    }
}

pub fn parse_expression(s : &str) -> Expression{
    let mut tokens = split_operator(s);
    let parsed = match_expressions(&mut tokens);
    print_expression(&parsed);
    evaluate_parsed_expression(&parsed)
}

pub fn print_expression(e : &ExpressionParsed){
    print!("{}", match &e.token {
        None => String::from(""),
        Some(Token::Identifier(s)) => s.clone(),
        Some(Token::Operator(o)) => String::from(o.to_char())
    });
    if !e.params.is_empty(){
    print!("(");
    for i in 0..e.params.len(){
        if i > 0 { print!(",");}
        print_expression(&e.params[i]);
    }
    print!(")");
    }
}