use crate::controller::Controller;
use crate::todo::Todo;
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
        println!("\t3. Help");
        println!("\t4. Quit");
        //todo: Metinleri text dosyadan okuyalım
        //todo: error durumlarında programdan çıkmasın devam etsin
        //todo: todo'ları json formatta kaydedelim
        //todo: Todo veri yapısına tarih bilgisi ekleyelim

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
                let mut input = String::new();
                println!("What do you want to-do?");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let id = controller.list().len() as u32 + 1;
                let todo = Todo::new(id, input.trim().to_string());
                controller.add(todo);
            }
            2 => {
                println!("Give me an ID of the to-do item you want to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");
                controller.complete(id);
            }
            3 => {
                println!("For example you can enter a todo something like this...");
                println!("Run 3 Km today...");
                println!("Go to supermarket and buy spagetti.");
            }
            4 => {
                println!("See yea!");
                return;
            }
            _ => {
                println!("I don't understand you. Try again");
            }
        }
    }
}
