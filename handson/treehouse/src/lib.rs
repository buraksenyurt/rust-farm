use std::io::stdin;

pub fn get_player_name() -> String {
    let mut player_name = String::new();
    stdin()
        .read_line(&mut player_name)
        .expect("Okuma işlemin hata. Programcını ara");
    player_name.trim().to_lowercase()
}

pub fn is_tree_house_friend(name: &str) -> bool {
    let visitors = ["ayşe", "mehmet", "burak"];
    for visitor in &visitors {
        if visitor == &name.trim().to_lowercase() {
            return true;
        }
    }
    false
}

#[derive(Debug)]
pub struct Visitor {
    pub name: String,
    pub action: VisitorAction,
    pub age: i8,
}

impl Visitor {
    pub fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    pub fn say_hello(&self) -> String {
        match &self.action {
            VisitorAction::Accept => format!("Ağaç evine hoşgeldin {}", self.name),
            VisitorAction::AcceptWithNote { note } => format!("Hoşgeşdin {}.\n{}", self.name, note),
            VisitorAction::Candidate => {
                format!("{} ağaç ev için ziyaretçi adayları arasında", self.name)
            }
            VisitorAction::Reject => {
                format!("Üzgünüm {}. Seni geri çevirmek zorundayım.", self.name)
            }
        }
    }
}

pub fn get_visitors() -> Vec<Visitor> {
    vec![
        Visitor::new("Burak", VisitorAction::Accept, 45),
        Visitor::new(
            "Ayşe",
            VisitorAction::AcceptWithNote {
                note: "Kitapların her zaman ki yerinde".to_string(),
            },
            35,
        ),
        Visitor::new("Mehmet", VisitorAction::Reject, 44),
    ]
}

#[derive(Debug)]
pub enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Reject,
    Candidate,
}

#[cfg(test)]
mod test {
    use crate::is_tree_house_friend;

    #[test]
    fn should_returns_true_for_authorized_friend() {
        let name = "buRAk";
        let actual = is_tree_house_friend(name);
        assert_eq!(actual, true);

        let name = "ayŞE";
        let actual = is_tree_house_friend(name);
        assert_eq!(actual, true);

        let name = "Mehmet";
        let actual = is_tree_house_friend(name);
        assert_eq!(actual, true);
    }
    #[test]
    fn should_returns_false_for_unauthorized_friend() {
        let name = "jakson";
        let actual = is_tree_house_friend(name);
        assert_eq!(actual, false);
    }
}
