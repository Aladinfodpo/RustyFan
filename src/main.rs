use std::ops;

struct Helice{
    curve : Curve
}


#[derive(Debug, Clone, Copy)]
struct Point{
    fields : [f32; Point::NUM_FIELD]
}

struct Curve {
    points : [Point; Curve::NUM_POINTS]
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
    pub const NUM_POINTS : usize = 12;

    fn create() -> Curve{
        Curve{points : core::array::from_fn( |j| {Point{fields :
                core::array::from_fn(|i| {
                    if i == 0 {j as f32} else {j as f32 * 100f32}
                })}
            })}
    }

    fn print(&self){
        for p in self.points{
            println!("Point debit = {}, pression = {}", p.fields[0], p.fields[1]);
        }
    }

    fn interpolated(&self, t : f32) -> Point{
        match self.points.iter().position(|&p| p.fields[0] > t){
            None => self.points[Curve::NUM_POINTS-1],
            Some(next) if next > 0 => {
                let prec = next - 1;

                Point::interpolate(&self.points[prec], &self.points[next], (t - self.points[prec].fields[0]) / (self.points[next].fields[0] - self.points[prec].fields[0]) )
            },
            Some(_) => self.points[0]
        }
        
    }
}


fn main() {
    let c = Curve::create();
    c.print();
    println!("Interpolated = {}", c.interpolated(3.5).fields[1]);
}
