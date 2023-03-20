use std::fs::{read, File};
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {}

fn load_facts() -> Option<Vec<Fact>> {
    if let Ok(lines) = read_lines("./norris_facts.dat") {
        let mut facts: Vec<Fact> = Vec::new();
        for (i, line) in lines.enumerate() {
            if let Ok(l) = line {
                facts.push(Fact::new(i as u32, l));
            }
        }
        return Some(facts);
    }
    None
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

struct Fact {
    id: u32,
    content: String,
}

impl Fact {
    pub fn new(id: u32, content: String) -> Self {
        Self { id, content }
    }
}

#[cfg(test)]
mod test{
    use crate::load_facts;

    #[test]
    fn read_file_lines_test(){
        let facts=load_facts();
        assert!(facts.is_some());
        assert!(facts.unwrap().len()>0);
    }
}
