use crate::todo::Todo;
use std::fs::{File, OpenOptions};
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
            if let Ok(t) = line {
                let parts = t.split('|').collect::<Vec<&str>>();
                let mut todo = Todo::new(parts[0].parse::<u32>().unwrap(), parts[1].to_string());
                match parts[2].parse::<bool>().unwrap() {
                    true => todo.completed = true,
                    false => todo.completed = false,
                }
                todos.push(todo);
            }
        }
    }
    todos
}

pub fn write_db(todo_list: &Vec<Todo>) -> bool {
    let file_name = "todos.dat";
    let mut file = OpenOptions::new()
        //.append(true)
        .write(true)
        .create(true)
        .open(file_name)
        .expect("Unable to open file");

    for t in todo_list.iter() {
        writeln!(file, "{}", t.format()).expect("Write error");
    }
    true
}
