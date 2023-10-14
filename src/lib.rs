#[warn(special_module_name)]



pub mod todo_Operations{



    pub fn command(mut string: String) -> String {
        // if the command is todo exit then we will return 0
        //* trim the command
        let commands:Vec<&str> = string.trim().split(" ").collect() ;
        println!("{:?}", commands) ;
        let code = String::new() ;
        for x in 0..commands.len() {
            println!("{}",x == 0 && commands[x] != "todo") ;
            if x == 0 && commands[x] != "todo" {
                println!("hey i got over here") ;
                return String::from("10") ;
            }else if x == 0 {
            }else{
                match commands[x] {
                    "add" =>{
                        //calling the add command function
                        println!("hoii") ;
                        return commands_check::add_command(commands[1..commands.len()].to_vec()) ;
                    },
                    "delete" => {
                        // calling the delete command function
                        return commands_check::delete_command(commands[1..commands.len()].to_vec()) ;
                    },
                    "edit" => {
                        //calling the edit command function
                        return commands_check::edit_command(commands[1..commands.len()].to_vec()) ;
                    }
                    "completed" => {
                        // calling the completed command function
                        return commands_check::completed_command(commands[1..commands.len()].to_vec()) ;
                    }
                    "get" => {
                        //calling the get command function
                        return commands_check::get_command(commands[1..commands.len()].to_vec()) ;
                    }
                    "exit" => {
                        return String::from("5") ;
                    }
                    _ => {
                        println!("haha") ;
                        return String::from("10") ; // invalid command
                    }
                }
            }
        }
        code
    }






    pub mod commands_check{
        pub fn add_command(vector: Vec<&str>) -> String{
            println!(" The add command vector was {:?}", vector) ;
            let mut code = String::new() ;
            if vector[0] == "add" && vector.len() == 4 {
                code.push_str("1") ;
                code.push_str(",") ;
                for x in 1..vector.len() {
                    code.push_str(vector[x]) ;
                    if x != vector.len()-1 {
                        code.push_str(",") ;
                    }
                }
                println!("The code was here {:?}", code) ;
                return code ;
            }
            println!("I'm here bro") ;
            return String::from("10") ; // invalid command
        }
        pub fn delete_command(vector: Vec<&str>)-> String{
            println!("{:?}", vector) ;
            if vector.len() != 2 {
                return String::from("10") ;
            }
            let mut code = String::from("2") ;
            code.push_str(",") ;
            code.push_str(vector[1]) ;
            return code;
        }
        pub fn edit_command(vector: Vec<&str>)-> String{
            let mut code = String::new() ;
            if vector.len() == 4 {
                code.push_str("3") ;
                code.push_str(",") ;
                code.push_str(vector[1]) ;
                code.push_str(",") ;
                code.push_str(vector[2]) ;
                code.push_str(",") ;
                code.push_str(vector[3]) ;
            }else{
                return String::from("10") ; // invalid command
            }
            code
        }

        pub fn completed_command(vector: Vec<&str>)-> String{
            let mut code = String::new() ;
            code.push_str("4") ;
            if vector.len() == 2 {
                code.push_str(",") ;
                code.push_str(vector[1]) ;
            }
            code
        }
        pub fn get_command(vector: Vec<&str>)-> String{
            let mut code = String::new() ;
            code.push_str("6") ;
            code.push_str(",") ;
            if vector.len() == 2 {
                code.push_str(vector[1]) ;
                code
            }else{
                return String::from("10") ;
            }
        }
    }

}



/*

    add -> 1 
    delete -> 2
    edit -> 3
    markasCompleted -> 4
    exit -> 5
    getCommand -> 6
*/



#[cfg(test)]
pub mod tests{
    #[test]
    pub fn testing_command_operations() {
        //* calling the add command function
        let vec1 = vec!["add", "taskname-1", "120", "12:32"] ;
        let string1 = crate::todo_Operations::commands_check::add_command(vec1) ;
        println!("The returned string was {}", string1) ;
        assert_eq!(string1, String::from("1,taskname-1,120,12:32")) ;
        //* let's try with the invalid commands
        assert_ne!(crate::todo_Operations::commands_check::add_command(["add", "taskname"].to_vec()), "1,taskname-1") ;
        //* calling the delete command function
        assert_eq!(crate::todo_Operations::commands_check::delete_command([
            "delete", "taskname"
        ].to_vec()), "2,taskname") ;
        assert_ne!(crate::todo_Operations::commands_check::delete_command([
            "delete", "taskname", "taskname"
        ].to_vec()), "2,taskname,taskname") ;
        //* calling the edit function
        assert_eq!(
            crate::todo_Operations::commands_check::edit_command([
                "edit", "taskname", "durationtime", "overall-time"
            ].to_vec()), "3,taskname,durationtime,overall-time"
        ) ;
        assert_ne!(
            crate::todo_Operations::commands_check::edit_command([
                "edit", "taskname", "overall-time"
            ].to_vec()), "3,taskname,overall-time"
        ) ;
        assert_eq!(
            crate::todo_Operations::commands_check::edit_command([
                "edit", "taskname",  "overall-time"
            ].to_vec()), "10"
        ) ;
        //* now testing the complete command function
        assert_eq!(
            crate::todo_Operations::commands_check::completed_command([
                "complete", "taskname"
            ].to_vec()), "4,taskname"
        ) ;
    }
}