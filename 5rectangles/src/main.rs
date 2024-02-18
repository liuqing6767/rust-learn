fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("{} {} {}", width1, height1, area(width1, height1));

    let rect1 = (30, 50);
    println!("{} {} {}", rect1.0, rect1.1, area1(rect1));

    let rect2 = Rect{
        width:30, 
        height:50,
    };
    println!("{} {} {}", rect2.width, rect2.height, area2(&rect2));

    println!("{} {} {}", rect2.width, rect2.height, rect2.area());
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn area2(rect: &Rect) -> u32 {
    rect.width * rect.height
}

fn area1(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
