use std::io;
use std::thread::sleep;
use std::time::Duration;

pub(crate) fn display(message: &str) -> () {
    println!("{}", message);
}

pub(crate) fn continue_action() -> (){
    display("Press ENTER to try again.");
    let mut input = String::new();
    if let Err(error) = io::stdin().read_line(&mut input){
        panic(&error.to_string())
    }
}
pub(crate) fn panic(message: &str) -> !{
    display(&message.to_string());
    loop {sleep(Duration::from_millis(100000))}
}