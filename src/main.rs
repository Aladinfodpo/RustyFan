struct Helice{
    curve : Curve
}


#[derive(Debug, Clone, Copy)]
struct Point{
    fields : [f32; 14]
}

struct Curve {
    points : [Point; 12]
}

impl Curve {
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
        let next = self.points.iter().position(|&p| p.fields[0] > t);
        let prec = next - 1;
        
    }
}


fn main() {
    let c = Curve::create();
    c.print();
}
