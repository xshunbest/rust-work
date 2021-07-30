struct rectangle{
    x:f32,
    y:f32
}

struct circle{
    r:f32,
}

struct triangle{
    d:f32,
    h:f32,
}

pub trait Area{
    fn get_area(&self)->f32{
        println!("get the area ...");
        0.0
    }
}

fn cal_Area<T:Area> (shape:T)->f32{
    shape.get_area()
}

impl Area for rectangle{
    fn get_area(&self)->f32
        {
        self.x * self.y
        }
}

impl Area for circle{
    fn get_area(&self)->f32
        {
        self.r * self.r * 3.1415 * 0.5
        }
}

impl Area for triangle{
    fn get_area(&self)->f32
        {
        self.d * self.h * 0.5
        }
}

fn main(){
     //第三题
    println!("question3：");

    let mut rec = rectangle{x:8.0,y:6.0};
    println!("rectangle area:{}",cal_Area(rec));

    let mut cir = circle{r:1.0};
    println!("circle area:{}",cal_Area(cir));

    let mut tri = triangle{d:3.0,h:2.0};
    println!("triangle area:{}",cal_Area(tri));
}
