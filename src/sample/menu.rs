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