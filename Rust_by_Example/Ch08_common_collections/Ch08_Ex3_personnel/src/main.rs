
use std::collections::HashMap;
use std::io;

// use colorized::*;
use console_utils::input;
// use console_menu::{Menu, MenuOption, MenuProps};

fn main() {

    let mut personnel_hash = Personnel::new();
    let dept: String;
    let login: String;

    (dept, login) = get_login_info(&personnel_hash);

    println!("dept: {dept}");
    println!("login: {login}");
    pause();
    personnel_hash.add_login(&dept, &login);
    personnel_hash.print_personnel();
    pause();
    personnel_hash.print_departments();
    pause();
    // launch_menu(&mut personnel_hash);


     personnel_hash.add_login("sales", "Pebbles");
     // println!("Add new dept and login:\t{:?}", personnel_hash);
    
     personnel_hash.add_login("admin", "Fred");
     // println!("Create first entry:\t{:?}", personnel_hash);
    
     personnel_hash.add_login("sales", "Bambam");
     // println!("Add new login:\t{:?}", personnel_hash);

     personnel_hash.add_login("admin", "Barney");
     // println!("Add new login:\t{:?}", personnel_hash);
    


     personnel_hash.print_personnel();
     pause();
     personnel_hash.print_departments();
     pause();

     launch_menu(& mut personnel_hash);

}

/////////////////////////////////////////////////////////////////////

fn pause(){

    let mut dummy = String::new();
    println
    !("\nPress enter to continue.");
    io::stdin().read_line(&mut dummy).expect("Failed to read dummy.");
}

fn launch_menu(pers: &mut Personnel) {
    let mut f_stop = false;

    let options = vec![
        "Add Person",
        "Print Personnel by Department",
        "Print Personnel list",
        "Exit"
    ];
    while !f_stop {
        let selected_indices = input::select("Use arrow keys to move up and down\nSpace bar to select an option and Enter to execute.", &options, false, false);

        match selected_indices {
            Some(indices) => {
                if indices[0]{
                    let dept: String;
                    let login: String;

                    (dept, login) = get_login_info(&pers);
                    pers.add_login(&dept, &login);

                } else if indices[1]{
                    pers.print_departments();
                    pause();

                } else if indices[2] {
                    pers.print_personnel();
                    pause();
                } else {
                    f_stop = true;
                };
            }
            None => {
                println!("No option selected.");
            }
        }
    }

}


fn get_login_info (pers: &Personnel) -> (String, String) {
    
    let mut dept = String::new();
    let mut login = String::new();

    println!("Current Departments:");
    // println!("\t{}\n", pers.get_depts_string());
    println!("please enter the department this person will be assigned to: ");  
    io::stdin().read_line(&mut dept).expect("Failed to read Department.");
    println!("");
    println!("Please enter the login name: ");
    io::stdin().read_line(&mut login).expect("Failed to read Name.");
    dept = dept.trim().to_string();
    login = login.trim().to_string();
    (dept, login)
}







/////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Personnel {
    pub dept_people: HashMap<String, Vec<String>>,
}

impl Personnel {
    
    fn new() -> Personnel {
        Self {
            dept_people: HashMap::new(),
        }
    }


    fn add_login(&mut self, dept: &str, login: &str) {
        if let Some(people) = self.dept_people.get_mut(dept) {
            people.push(login.to_string());
        } else {
            let people = vec!(login.to_string());
            self.dept_people.insert(dept.to_string(), people);
        }
    }

    fn print_personnel(&mut self) {

        let mut people: Vec<String> = Vec::new();
        let mut keys: Vec<String> = Vec::new();

        for dept in self.dept_people.keys(){
            keys.push(dept.clone());
        }
        keys.sort();

        println!();
        for dept in keys {
            if let Some(list) = self.dept_people.get_mut(&dept) {
           
                for name in list {
                    let mut entry = name.clone();
                    entry.push_str("\t");
                    entry.push_str(&dept);
                    people.push(entry);
                }
            }
        }
        people.sort();
        for entry in people {
            println!("{entry}");
        }
    }
    
    
    fn print_departments(&mut self) {
        let mut keys: Vec<String> = Vec::new();

        for dept in self.dept_people.keys(){
            keys.push(dept.clone());
        }
        keys.sort();

        println!();
        for dept in keys {
            if let Some(list) = self.dept_people.get_mut(&dept){
                println!("{dept}");
                println!("------------------------------------------------");

                list.sort();
                for name in list {
                    println!("{name}");
                }
                 println!();
           }
        }
    }
    # [allow (dead_code)]
    fn department_exists(&self, dept: &str) -> bool {
        for key in self.dept_people.keys(){
            if dept == key {
                return true;
            }    
        }

        false
    }


    fn get_depts(&self) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();

        for dept in self.dept_people.keys(){
            keys.push(dept.clone());
        }

        keys
    }
    # [warn (dead_code)]

    fn get_depts_string(&self) -> String {
        let mut keys: String = String::new();

        for dept in self.dept_people.keys(){
            keys.push_str(dept);
            keys.push_str(", ");
        }
        keys.pop();     // Get rid of trailing comma and space
        keys.pop();
 
        keys
    }
}