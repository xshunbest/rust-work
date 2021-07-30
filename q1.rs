
//第一题 交通信号灯
trait Time{
    fn light_time(&self)->u32{
        0
    }
}

enum TrafficLight{
    Red,
    Green,
    Yellow,
}

impl Time for TrafficLight{
    fn light_time(&self)->u32{
        match self{
            TrafficLight::Red => 20 ,
            TrafficLight::Green => 40 ,
            TrafficLight::Yellow => 2 ,
        }
    }
}

fn main(){
    //第一题
    let light_red = TrafficLight::Red;
    let light_green = TrafficLight::Green;
    let light_yellow = TrafficLight::Yellow;
    println!("question1：");
    println!("Time of light_red: {}",light_red.light_time());
    println!("Time of light_green: {}",light_green.light_time());
    println!("Time of light_yellow: {}",light_yellow.light_time());
}
