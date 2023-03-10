/*
   Örnekte Binary Tree modeli işlenmektedir. Binary Tree kuralları şöyledir.
   - Tek bir Root Node olabilir.
   - Her node en fazla iki alt node'a bağlanabilir.
   - Root ile herhangibir alt node arasında tek bir yol(path) söz konusu olabilir.
*/
use std::cell::RefCell;
use std::rc::Rc;

fn main() {}

struct Person {
    full_name: String,
    age: u32,
}

// RefCell sadece immutable referans içerse bile değerin değiştirilebilmesi için(interior mutability)
// Rc ise Node değerinin sahipliğinin paylaşılabilmesi(Shared Ownership) içindir.
type NodeRef = Rc<RefCell<Node>>;

struct Node {
    person: Person,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

fn average_age(root: NodeRef) -> f32 {
    let mut sum = 0u32;
    let mut stack = vec![root];
    while !stack.is_empty() {
        let current: Rc<RefCell<Node>> = stack.pop().unwrap();
        sum += current.borrow().person.age;
        if let Some(right) = &current.borrow().right {
            stack.push(right.to_owned());
        };
        if let Some(left) = &current.borrow().left {
            stack.push(left.to_owned());
        };
    }
    sum as f32 / stack.len() as f32
}
