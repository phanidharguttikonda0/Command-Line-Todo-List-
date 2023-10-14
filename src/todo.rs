pub mod todo {
    use std::{collections::HashMap, rc::Rc, cell::RefCell};

    #[derive(Debug)]
pub struct Todo {
    name: String,
    durationtime: u32,
    starttime: String,
} 

impl Todo {
    fn new(name:String, durationtime: u32, starttime: String) -> Todo {
        Todo { name, durationtime, starttime}
    }
}

pub struct todo_list {
    current_tasks1 : Vec<Rc<RefCell<Todo>>>,
    current_tasks2 : HashMap<String, Rc<RefCell<Todo>>>, //maintaining the taskname
    deleted_tasks1 : Vec<Rc<RefCell<Todo>>>,
    deleted_tasks2 : HashMap<String, Rc<RefCell<Todo>>>,
    completed_tasks1 : Vec<Rc<RefCell<Todo>>>,
    completed_tasks2 : HashMap<String, Rc<RefCell<Todo>>>,
}


    impl todo_list {
        pub fn new() -> todo_list {
            todo_list {
                current_tasks1 : Vec::new(),
                current_tasks2 : HashMap::new(),
                deleted_tasks1 : Vec::new(),
                deleted_tasks2 : HashMap::new(),
                completed_tasks1 : Vec::new(),
                completed_tasks2 : HashMap::new()
            }
        }
        pub fn get_current_tasks(&self) -> &Vec<Rc<RefCell<Todo>>> {
            &self.current_tasks1 
        }
        pub fn get_deleted_tasks(&self) -> &Vec<Rc<RefCell<Todo>>> {
            &self.deleted_tasks1 
        }
        pub fn get_completed_tasks(&self) -> &Vec<Rc<RefCell<Todo>>> {
            &self.completed_tasks1
        }
        pub fn add_task(&mut self, name: String, durationtime: u32, starttime: String) {
            let todo = Rc::new(RefCell::new(Todo::new(name, durationtime, starttime))) ;
            self.current_tasks1.push(todo.clone()) ;
            self.current_tasks2.insert(todo.clone().borrow_mut().name.clone(), todo.clone()) ;
            //created an task and added that task to the current_tasks
        }
        pub fn delete_task(&mut self, name: String) {
            // we need to find the task and then we have to delete the task
            for x in 0..self.current_tasks1.len() {
                // we can only delete from the current tasks
                if name == self.current_tasks1[x].clone().borrow_mut().name.clone() {
                    self.deleted_tasks1.push(self.current_tasks1[x].clone()) ;
                            // now deleting from the hashmap
                    self.deleted_tasks2.insert(String::from(&name),self.current_tasks1[x].clone()) ;
                    self.current_tasks1.remove(x) ;
                    break ; // the element will be removed from the current tasks and added to the deleted tasks
                }
            }
        }

        pub fn edit_task(&mut self,name: String, durationtime: u32, starttime: String) {
            for x in 0..self.current_tasks1.len() {
                if name == self.current_tasks1[x].clone().borrow_mut().name.clone() {
                    if durationtime != 0 {
                        self.current_tasks1[x].clone().borrow_mut().durationtime = durationtime ;
                    } println!("hahaha {}", starttime) ;
                    if String::from(&starttime).len() != 0 {
                        println!("hurray") ;
                        self.current_tasks1[x].clone().borrow_mut().starttime = starttime ;
                    }
                    break;
                }
            }
        }

        pub fn completed_task(&mut self, name:String) {
            // adding the current tasks to the completed tasks
            for x in 0..self.current_tasks1.len() {
                // we can only delete from the current tasks
                if name == self.current_tasks1[x].clone().borrow_mut().name.clone() {
                    self.completed_tasks1.push(self.current_tasks1[x].clone()) ;
                            // now deleting from the hashmap
                    self.completed_tasks2.insert(String::from(&name),self.current_tasks1[x].clone()) ;
                    self.current_tasks1.remove(x) ;
                    break ; // the element will be removed from the current tasks and added to the deleted tasks
                }
            }
        }
    } 
} 