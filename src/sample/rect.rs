#[derive(Debug)]
pub struct Rect{
    pub height: u32, 
    pub width: u32,
}

impl Rect {
    pub fn new(height: u32, width: u32)-> Self{
        Self { height, width }
    }

    pub fn area(&self) -> u32{
        self.height * self.width
    }
}

pub fn main1(){
    let rect0 = Rect::new(5, 7);
    println!(
        "The rectange we given is {:?} and the area is {:?}", 
        &rect0, 
        rect0.area()
    );
}
