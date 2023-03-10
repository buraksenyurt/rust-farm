use crate::controller::Controller;
use crate::repository::write_db;
use std::io;

mod controller;
mod repository;
mod test;
mod todo;

fn main() {
    let mut controller = Controller::default();

    loop {
        println!("\tYour Todos");
        println!("\t0. List");
        println!("\t1. Add");
        println!("\t2. Complete");
        println!("\t3. Delete");
        println!("\t4. Help");
        println!("\t5. Quit");

        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");
        let selected_option = user_choice.trim().parse::<u8>().expect("Invalid input");

        match selected_option {
            0 => {
                for c in controller.list().iter() {
                    println!("{}", c);
                }
            }
            1 => {
                if controller.list().len() >= 5 {
                    println!(
                        "Maximum task capacity reached. You can think to delete someone of them."
                    );
                    continue;
                }
                let mut input = String::new();
                println!("What do you want to-do?");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let task_id = controller.add(input.trim().to_string());
                println!("Task number {} has been added to the list", task_id);
            }
            2 => {
                println!("Give me an ID of the to-do item you want to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");
                controller.complete(id);
            }
            3 => {
                println!("Give me an ID of the to-do item you want to delete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");
                controller.delete(id);
            }
            4 => {
                println!("For example you can enter a todo something like this...");
                println!("Run 3 Km today...");
                println!("Go to supermarket and buy spagetti.");
            }
            5 => {
                println!("See yea!");
                if !write_db(controller.list()) {
                    println!("Uncompleted save progress");
                }
                return;
            }
            _ => {
                println!("I can't understand you. Please try again...");
            }
        }
    }
}
