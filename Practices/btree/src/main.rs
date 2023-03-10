/*
   Örnekte Binary Tree modeli işlenmektedir. Binary Tree kuralları şöyledir.
   - Tek bir Root Node olabilir.
   - Her node en fazla iki alt node'a bağlanabilir.
   - Root ile herhangibir alt node arasında tek bir yol(path) söz konusu olabilir.
*/
use std::cell::RefCell;
use std::rc::Rc;

fn main() {}

// RefCell sadece immutable referans içerse bile değerin değiştirilebilmesi için(interior mutability)
// Rc ise Node değerinin sahipliğinin paylaşılabilmesi(Shared Ownership) içindir.
pub type CellNodeRef = Rc<RefCell<CellNode>>;

#[derive(Debug, Clone)]
pub struct CellNode {
    distance: f32,
    pub left: Option<CellNodeRef>,
    pub right: Option<CellNodeRef>,
}

impl CellNode {
    pub fn new(distance: f32) -> Self {
        Self {
            distance,
            left: None,
            right: None,
        }
    }
}

// Ortalama mesafeyi bulmak için kullanılan fonksiyon
pub fn calculate_average_distance(root: CellNodeRef) -> f32 {
    let mut sum = 0f32;
    let mut counter = 0;
    let mut stack = vec![root];
    // Last In First Out ilkesine göre çalışabilecek bir vectör kullanılıyor(stack diyebiliriz)
    // Döngü root Node ile başlıyor ve onun sol veya sağında Node olup olmamasına göre
    // stack vektörü sürekli besleniyor.
    while !stack.is_empty() {
        // O anki Node değerini alıyoruz.
        let current: Rc<RefCell<CellNode>> = stack.pop().unwrap();
        // mesafeleri topluyoruz
        sum += current.borrow().distance;
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
    use crate::{calculate_average_distance, CellNode};
    use std::cell::{RefCell};
    use std::rc::Rc;

    #[test]
    pub fn calculate_average_age_test() {
        let mut root_node = CellNode::new(54.45);
        let mut node_a = CellNode::new(54.49);
        let mut node_b = CellNode::new(19.);
        let node_c = CellNode::new(21.45);
        let node_d = CellNode::new(98.90);
        let node_e = CellNode::new(44.44);
        let mut node_f = CellNode::new(66.62);
        let mut node_g = CellNode::new(18.35);

        node_g.right = Some(Rc::new(RefCell::new(node_e)));
        node_f.right = Some(Rc::new(RefCell::new(node_g)));
        node_a.right = Some(Rc::new(RefCell::new(node_f)));
        root_node.left = Some(Rc::new(RefCell::new(node_a)));
        node_b.left = Some(Rc::new(RefCell::new(node_c)));
        node_b.right = Some(Rc::new(RefCell::new(node_d)));
        root_node.right = Some(Rc::new(RefCell::new(node_b)));

        let expected = 47.2125;
        let actual = calculate_average_distance(Rc::new(RefCell::new(root_node)));
        assert_eq!(expected, actual);
    }
}
