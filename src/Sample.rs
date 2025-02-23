#[derive(Debug)]
struct Rect{
    height: u32, 
    width: u32
}

impl Rect {
    fn new(height: u32, width: u32)-> Self{
        Self { height, width }
    }

    fn area(&self) -> u32{
        self.height * self.width
    }
}

pub fn main2(){
    let rect1 = Rect::new(5, 7);
    println!(
        "The rectange we given is {:?} and the area is {:?}", 
        &rect1, 
        rect1.area()
    );
}


#[derive(Debug)]
enum MainChoice {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MainChoice, String>{
    match input {
        "MainMenu" => Ok(MainChoice::MainMenu),
        "Start" => Ok(MainChoice::Start),
        "Quit" => Ok(MainChoice::Quit),
        _ => Err("Main choice not found".to_owned()),
    }
}

fn print_choice(choice: &MainChoice){
    println!("choice = {:?}", choice);
}

pub fn main(){
    let choice: MainChoice = MainChoice::MainMenu;
    print_choice(&choice);

    let choice: Result<MainChoice, String> = get_choice("MainMen");
    match choice {
        Ok(some_thing) => print_choice(&some_thing),
        Err(e) => println!("error = {:?} ", e)
    }

}