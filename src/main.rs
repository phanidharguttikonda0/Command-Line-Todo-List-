pub mod lib;

use std::io ;
use todolist::todo_Operations ;
fn main() {
    // firstly get the command line arguments
    println!("To-Do List Application Started") ;
    loop {
        let mut command = String::new() ;
        println!(">>") ;
        io::stdin().read_line(&mut command).expect("Unable to read the input") ;

        //* we are going to check whether the command is right or wrong
        let mut operations:String = todo_Operations::command(command) ;
        let command_operations:Vec<&str> = operations.split(",").collect() ;
        println!() ;
    }
}
