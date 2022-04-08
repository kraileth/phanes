use std::io;

use crate::datamanager::*;
use crate::datamanager::db::Database;

pub fn run () {
    banner();
    loop {
        display_main_menu();
        let input = get_number_option();
        let choice = match input {
            Some(i) => {
                if i > 4 || i < 0 {
                    continue;
                }
                i
            }
            None => continue,
        };

        if choice == 1 {
            display_tasks_menu();
        } else if choice == 2 {
            manage_tasks();
        } else if choice == 3 {
            // TODO
        } else if choice == 4 {
            return;
        }
    }
}

fn banner() {
    println!("Phanes task manager v0.0.0\n");
}

fn display_main_menu() {
    println!("Main Menu:");
    println!("\t1. See Tasks");
    println!("\t2. Manage Tasks");
    println!("\t3. Manage Categories");
    println!("\t4. quit");
    println!("Select an option [0-4]: ");
}

fn display_tasks_menu() {
    println!("Options for displaying tasks:");
    println!("\t1. See all opened tasks");
    println!("\t2. See all in-progress tasks");
    println!("\t3. See all closed tasks");
    println!("\t4. Back to main menu");

    let input = get_number_option();
    let choice = match input {
        Some(i) => {
            if i > 4 {
                return
            }
            if i < 0 {
                return
            }
            i
        },
        None => return,
    };

    // TODO: Need to work out returning query results
    match choice {
        4 => return,
        1 => { // see all opened tasks
            match task::get_task_by_status(&Database::new("/Users/toddmartin/Projects/phanes/phanes/src/data/data.db".to_string()), 1) {
                Ok(results) => {
                    for i in results.iter() {
                        println!("{i}");
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        },
        2 =>  {
            match task::get_task_by_status(&Database::new("/Users/toddmartin/Projects/phanes/phanes/src/data/data.db".to_string()), 2) {
                Ok(results) => {
                    for i in results.iter() {
                        println!("{i}");
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }, // See all in-progress tasks
        3 => {
            match task::get_task_by_status(&Database::new("/Users/toddmartin/Projects/phanes/phanes/src/data/data.db".to_string()), 3) {
                Ok(result) => {
                    for i in result.iter() {
                        println!("{i}");
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }, // See all closed tasks
        _ => { // return
            println!("Not a valid option");
            return;
        },
    };
}

fn display_category_menu() {
    println!("Options for managing categores:");
    println!("\t1. Add a category");
    println!("\t2. Delete a category");
    println!("\t3. List categories")

    let choice : i64 = match get_number_option() {
        Some(i) => {
            if i < 1 {
                println!("Error: Invalid number");
                return;
            } else if i > 3 {
                println!("Error: Invalid number");
                return;
            }
            i
        },
        None => {
            println!("Error: Not a valid input");
            return;
        },
    };

    match choice {
        1 => {

        },
        2 => {

        },
        3 => {
            category::get_all_categories();
        },
        _ =>return,
    }
}

fn manage_tasks() {
    println!("Options for managing tasks");
    println!("\t1. Add a task");
    println!("\t2. Delete a task");
    println!("\t3. Move open task to in-progress");
    println!("\t4. Move in-progress task to closed");
    println!("\t5. Assign task a category");
    println!("\t6. Return to main menu");
    println!("Enter choice");

    let number: i64 = match get_number_option() {
        Some(i) => {
            if i > 6 {
                println!("Not a valid option");
                return;
            } else if i < 1 {
                println!("Not a valid option");
            }
            i
        },
        None => {
            println!("Not a valid option");
            return;
        }
    };

    match number {
        1 => add_task(), // add a task
        2 => { // Delete a task
            match ask_question_number("Enter ID of task to delete:") {
                Some(i) => {
                    match task::remove_task(&Database::new("../data/data.db".to_string()), i) {
                        Ok(_) => println!("Success, task removed!"),
                        Err(_) => println!("Task not able to be removed"),
                    }
                },
                None => return,
            };
        },
        3 => { // Move open task to in progress
            match ask_question_number("Enter ID of task to move to in-progress:") {
                Some(i) => {
                    match task::change_task_status(&Database::new("/Users/toddmartin/Projects/phanes/phanes/src/data/data.db".to_string()), i, 2) {
                        Ok(_) => println!("Success, task moved to in-progress"),
                        Err(e) => println!("{:?}", e),
                    }
                },
                None => return,
            };
        },
        4 => { // Move task to closed
            match ask_question_number("Enter ID of task to move to closed:") {
                Some(i) => {
                    match task::change_task_status(&Database::new("../data/data.db".to_string()), i, 3) {
                        Ok(_) => println!("Success, task moved to closed"),
                        Err(_) => println!("Error, task not able to move to closed"),
                    }
                },
                None => return,
            };
        },
        5 => { // asign task a category
            let task_id = match ask_question_number("Enter ID of a task to assign a category:") {
                Some(i) => i,
                None => {
                    println!("Not a valid input");
                    return;
                },
            };
            let category_id = match ask_question_number("Enter a category to assign to:") {
                Some(i) => i,
                None => {
                    println!("Not a valid input");
                    return;
                },
            };
            match task::change_task_category(&Database::new("/Users/toddmartin/Projects/phanes/phanes/src/data/data.db".to_string()), task_id, category_id) {
                Ok(_) => println!("Operation successful"),
                Err(e) => println!("{:?}", e),
            }
        }, // return
        _ => {
            return;
        }
    };
}

fn add_task() {
    let title = match ask_question("What is the task title? ") {
        Some(s) => s,
        None => return,
    };
    let desc = match ask_question("Describe the task: ") {
        Some(s) => s,
        None => return,
    };
    let status: i64 = 1;
    // Eventually handle category better
    let category = 1;
    match task::add_tasks(&Database::new("/Users/toddmartin/Projects/phanes/phanes/src/data/data.db".to_string()), title, desc, status, category) {
        Ok(_) => println!("Success, task added!"),
        Err(e) => println!("{:?}", e),
    };
}

fn ask_question_number(question: &str) -> Option<i64> {
    println!("{question}");
    match get_number_option() {
        Some(i) => return Some(i),
        None => {
            println!("Not a valid option");
            return None
        },
    }
}

fn ask_question(question: &str) -> Option<String> {
    println!("{question}");
    get_string_input()
}

fn get_string_input() -> Option<String> {
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(s) => {
            return Some(input)
        },
        Err(_e) =>  {
            println!("Input invalid.");
            return None
        },
    }
}


fn get_number_option() -> Option<i64> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_s) => {
            match input.trim().parse::<i64>() {
                Ok(i) => Some(i),
                Err(_e) => None,
            }
        },
        Err(_e) => None,
    }
}