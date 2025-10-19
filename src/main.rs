mod equation;
use core::f32;
use std::{ops};

use crate::equation::parse_expression;

#[derive(Debug, Clone, Copy)]
struct Point{
    fields : [f32; Point::NUM_FIELD]
}

struct Curve {
    points : Vec<Point>
}

impl ops::Mul<Point> for f32{
    type Output = Point;
    fn mul(self, p: Point) -> Self::Output {
            Point{ fields : core::array::from_fn(|i| p.fields[i] * self )}
    }
}

impl ops::Add<Point> for Point{
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point{fields : core::array::from_fn(|i| self.fields[i] + rhs.fields[i])}
    }
}

impl Point{
    pub const NUM_FIELD : usize = 14;

    fn interpolate(p1 : &Point, p2 : &Point, t : f32) -> Point{
        t * (*p2) + (1f32-t)*(*p1)
    }    
}

impl Curve {
    fn sample_from_function(e : equation::Expression, n : i32, start : f32, end : f32) -> Curve{
        Curve{points : (0..n).map( |j| {Point{fields :
                core::array::from_fn(|i| {
                    let x = start + (end - start) * (j as f32) / ((n-1) as f32);
                    if i == 0 {x} else {e.evaluate(x)}
                })}
            }).collect()}
    }

    fn create(n : i32) -> Curve{
        Curve{points : (0..n).map( |j| {Point{fields :
                core::array::from_fn(|i| {
                    if i == 0 {j as f32} else {j as f32 * 100f32}
                })}
            }).collect()}
    }

    fn print(&self){
        for p in &self.points{
            println!("Point debit = {}, pression = {}", p.fields[0], p.fields[1]);
        }
    }

    fn interpolated(&self, t : f32) -> Point{
        match self.points.iter().position(|&p| p.fields[0] > t){
            None => *self.points.last().expect("Empty curve"),
            Some(next) if next > 0 => {
                let prec = next - 1;

                Point::interpolate(&self.points[prec], &self.points[next], (t - self.points[prec].fields[0]) / (self.points[next].fields[0] - self.points[prec].fields[0]) )
            },
            Some(_) => *self.points.first().expect("Empty curve")
        }
    }
}


fn main() {
    //let b = Curve::create(10);
    //b.print();
    println!("Expression evaluated to {}", parse_expression("sin(x)^2 + cos(x)^2").evaluate(30.141592));
    //let c = Curve::sample_from_function(equation::Expression::create_from_function(f32::sin), 10, 0f32, f32::consts::PI);
    //c.print();
    //println!("Interpolated = {}", c.interpolated(0.5).fields[1]);
}
