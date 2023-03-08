use crate::todo::Todo;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_db() -> Vec<Todo> {
    let mut todos: Vec<Todo> = Vec::new();
    if let Ok(lines) = read_lines("todos.dat") {
        for line in lines {
            if let Ok(_t) = line {
                let todo = Todo::new(1, "Test".to_string());
                todos.push(todo);
            }
        }
    }
    todos
}

pub fn write_db(todo_list: &Vec<Todo>) -> bool {
    let file_name = "todos.dat";
    match File::create(file_name) {
        Ok(mut f) => {
            for t in todo_list.iter() {
                writeln!(f, "{}", t.to_string()).expect("Write error");
            }
            true
        }
        Err(_) => false,
    }
}
