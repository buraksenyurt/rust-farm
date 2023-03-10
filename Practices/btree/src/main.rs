/*
   Örnekte Binary Tree modeli işlenmektedir. Binary Tree kuralları şöyledir.
   - Tek bir Root Node olabilir.
   - Her node en fazla iki alt node'a bağlanabilir.
   - Root ile herhangibir alt node arasında tek bir yol(path) söz konusu olabilir.
*/
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut boss_node = CellNode::new("Boss".to_string(), 45.);
    let mut bill = CellNode::new("Bill".to_string(), 54.49);
    let mut jenny = CellNode::new("Jenny".to_string(), 19.);
    let jean_claud = CellNode::new("Jean Claud".to_string(), 21.45);
    let agness = CellNode::new("Agness".to_string(), 98.90);
    let mikaela = CellNode::new("Mikaela".to_string(), 44.44);
    let mut loran = CellNode::new("Loran".to_string(), 66.62);
    let mut mam = CellNode::new("Mam".to_string(), 18.35);

    mam.right = Some(Rc::new(RefCell::new(mikaela)));
    loran.right = Some(Rc::new(RefCell::new(mam)));
    bill.right = Some(Rc::new(RefCell::new(loran)));

    boss_node.left = Some(Rc::new(RefCell::new(bill)));
    jenny.left = Some(Rc::new(RefCell::new(jean_claud)));
    jenny.right = Some(Rc::new(RefCell::new(agness)));
    boss_node.right = Some(Rc::new(RefCell::new(jenny)));

    println!(
        "Average experience of tribe {}",
        calculate_average_experience(boss_node)
    );
}

// RefCell sadece immutable referans içerse bile değerin değiştirilebilmesi için(interior mutability)
// Rc ise Node değerinin sahipliğinin paylaşılabilmesi(Shared Ownership) içindir.
pub type CellNodeRef = Rc<RefCell<CellNode>>;

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    experience: f32,
}
impl Person {
    pub fn new(name: String, experience: f32) -> Self {
        Self { name, experience }
    }
}

#[derive(Debug, Clone)]
pub struct CellNode {
    person: Person,
    pub left: Option<CellNodeRef>,
    pub right: Option<CellNodeRef>,
}

impl CellNode {
    pub fn new(name: String, experience: f32) -> Self {
        Self {
            person: Person::new(name, experience),
            left: None,
            right: None,
        }
    }
}

// Ortalama mesafeyi bulmak için kullanılan fonksiyon
pub fn calculate_average_experience(node: CellNode) -> f32 {
    let root = Rc::new(RefCell::new(node));
    let mut sum = 0f32;
    let mut counter = 0;
    let mut stack = vec![root];
    // Last In First Out(LIFO) ilkesine göre çalışabilecek bir vectör kullanılıyor(stack diyebiliriz)
    // Döngü root Node ile başlıyor ve onun sol veya sağında Node olup olmamasına göre
    // stack vektörü sürekli besleniyor.
    while !stack.is_empty() {
        // O anki Node değerini alıyoruz.
        let current: Rc<RefCell<CellNode>> = stack.pop().unwrap();
        // mesafeleri topluyoruz
        sum += current.borrow().person.experience;
        // Eğer sağ tarafta bir node varsa yığına o Node'u ekliyoruz.
        if let Some(right) = &current.borrow().right {
            stack.push(right.to_owned());
        };
        // Eğer sol tarafta bir Node varsa bu kez o Node'u ekliyoruz.
        if let Some(left) = &current.borrow().left {
            stack.push(left.to_owned());
        };
        counter += 1;
    }
    sum as f32 / counter as f32
}

#[cfg(test)]
mod test {
    use crate::{calculate_average_experience, CellNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    pub fn calculate_average_age_test() {
        let mut boss_node = CellNode::new("Boss".to_string(), 45.);
        let mut bill = CellNode::new("Bill".to_string(), 54.49);
        let mut jenny = CellNode::new("Jenny".to_string(), 19.);
        let jean_claud = CellNode::new("Jean Claud".to_string(), 21.45);
        let agness = CellNode::new("Agness".to_string(), 98.90);
        let mikaela = CellNode::new("Mikaela".to_string(), 44.44);
        let mut loran = CellNode::new("Loran".to_string(), 66.62);
        let mut mam = CellNode::new("Mam".to_string(), 18.35);

        mam.right = Some(Rc::new(RefCell::new(mikaela)));
        loran.right = Some(Rc::new(RefCell::new(mam)));
        bill.right = Some(Rc::new(RefCell::new(loran)));
        boss_node.left = Some(Rc::new(RefCell::new(bill)));

        jenny.left = Some(Rc::new(RefCell::new(jean_claud)));
        jenny.right = Some(Rc::new(RefCell::new(agness)));
        boss_node.right = Some(Rc::new(RefCell::new(jenny)));

        let expected = 46.031254;
        let actual = calculate_average_experience(boss_node);
        assert_eq!(expected, actual);
    }
}
