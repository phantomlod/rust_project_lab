//CRUD Application
//Create
//Read
//Update
//Delete

use std::io;
use std::collections::HashMap;
#[derive(Debug,Clone)]
pub struct Student{
    name : String,
    age :i32,
}

pub struct Students {
    class: HashMap<String,Student>,
}

impl Students {
    fn new() -> Self {
        Self {class: HashMap::new() }
    }
    fn add(&mut self, student : Student ) {
        self.class.insert(student.name.to_string(), student);
    }
    fn view_all(&self) -> Vec<&Student> {
        self.class.values().collect()
    }
    fn remove(&mut self, name : &str) -> bool {
        self.class.remove(name).is_some()
    }
    fn edit(&mut self, name : &str, age : i32) -> bool{
        match self.class.get_mut(name) {
            Some(name) => {
                name.age = age;
                true
            }
            None => false,
        }
    }
}

mod manager {
    use crate::*;
    pub fn add_student (students : &mut Students) {
        println!("Enter name of Student");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        let age = match get_int() {
            Some(input) => input,
            None => return,
        };
        let student = Student{name,age};
        students.add(student)

    }
    pub fn view(students : &Students){
        for student in students.view_all(){
            println!("{:?}",student);
        }
    }
    pub fn remove_student(students: &mut Students){
        for student in students.view_all(){
            println!("{:?}",student);
        }
        let name = match get_input() {
            Some(input)=> input,
            None => return,
        };
        if students.remove(&name){
            println!("ok");
        } else {
            println!("not found");
        }
    }
    pub fn edit_student(students: &mut Students){
        for student in students.view_all(){
            println!("{:?}",student);
        }
        let name = match get_input() {
            Some(input)=> input,
            None => return,
        };
        let age = match get_int() {
            Some(input)=> input,
            None => return,
        };
        if students.edit(&name, age) {
            println!("oke");
        } else {
            println!("not found");
        }
    }
}

fn get_input() -> Option<String>{
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Let try again");
    }
    let input = buffer.trim().to_owned();
    if input == ""{
        None
    } else {
        Some(input)
    }
}

fn get_int() -> Option<i32> {
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };
    let parse_input: Result<i32, _> =input.parse();
    match parse_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }

}

enum Manager {
    addStudent,
    viewStudent,
    editStudent,
    deleteStudent,
}

impl Manager {
    fn show() {
        println!("1. addStudent ");
        println!("2. viewStudent ");
        println!("3. editStudent ");
        println!("4. deleteStudent ");
        println!("Please choice :") ;        
    }
    fn choice(input : &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::addStudent),
            "2" => Some(Manager::viewStudent),
            "3" => Some(Manager::editStudent),
            "4" => Some(Manager::deleteStudent),
            _ => None,
        }
    }
}

fn main() {
    let mut students =Students::new();
    loop {
        Manager::show();
        let input = get_input().expect("Please enter your data:");
        match Manager::choice(&input.as_str()) {
            Some(Manager::addStudent) => manager::add_student(&mut students),
            Some(Manager::viewStudent) => manager::view(&mut students),
            Some(Manager::editStudent) => manager::edit_student(&mut students),
            Some(Manager::deleteStudent) => manager::remove_student(&mut students),
            None => return,
        }
    }   
}
