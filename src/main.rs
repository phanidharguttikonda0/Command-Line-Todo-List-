mod lib;
use std::io ;
use todolist::todo_Operations ;
mod todo;
use crate::todo::todo::todo_list ;
fn main() {
    // creating an instance of the todo_list
    let mut todoList = todo_list::new() ;
    // firstly get the command line arguments
    println!("To-Do List Application Started") ;
    loop {
        let mut command = String::new() ;
        println!(">>") ;
        io::stdin().read_line(&mut command).expect("Unable to read the input") ;

        //* we are going to check whether the command is right or wrong
        let mut operations:String = todo_Operations::command(command) ;
        println!("The Command was {}", operations) ;
        let command_operations:Vec<&str> = operations.split(",").collect() ;
        if command_operations[0] == "10" {
            println!("Invalid Command Please try again") ;
        }else if command_operations[0] == "5" {
            break;
        }else{
            command_calls(&mut todoList, command_operations) ;
        }
        println!() ;
    }
}

pub fn command_calls(todo_list1: &mut todo_list, command_operations:Vec<&str>) {
    let x  = 0 ;
    if command_operations[x].contains("1") {
        // going to call the add function
        println!("{}", command_operations[x]) ;
        let a:u32 = String::from(command_operations[2]).parse().expect("unable to parse") ;
        println!("The a value is {}",a) ;
        todo_list1.add_task(String::from(command_operations[1]),a, String::from(command_operations[3])) ;
        println!("Successfully added the task") ;
    }
    else if command_operations[x].contains("2") {
        // going to call the delete function
        todo_list1.delete_task(String::from(command_operations[1])) ;
        println!("Successfully deleted the task ") ;
    }
    else if command_operations[x].contains("3") {
        // going to call the edit function
        let a = String::from(command_operations[2]).parse().expect("") ;
        todo_list1.edit_task(String::from(command_operations[1]),a, String::from(command_operations[3]) ) ;
        println!("Successfully edited the task") ;
    }
    else if command_operations[x].contains("4") {
        // going to call the completed function
        todo_list1.completed_task(String::from(command_operations[1])) ;
        println!("Successfully Marked as Completed") ;
    }
    else {
        // going to call the get function
        if command_operations[1] == "current-tasks" {
            println!("{:?}", todo_list1.get_current_tasks()) ;
        }
        else if command_operations[1] == "deleted-tasks" {
            println!("{:?}", todo_list1.get_deleted_tasks()) ;
        }
        else{
            println!("{:?}", todo_list1.get_completed_tasks()) ;
        }
    }
}
