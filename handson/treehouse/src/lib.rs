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
    pub wellcome: String,
}

impl Visitor {
    pub fn new(name: &str, wellcome: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            wellcome: wellcome.to_string(),
        }
    }

    pub fn say_hello(&self) -> String {
        format!("Merhaba {}. {}", self.name, self.wellcome)
    }
}

pub fn get_visitors() -> Vec<Visitor> {
    vec![
        Visitor::new("Burak", "Ağaçevine hoşgeldin."),
        Visitor::new("Ayşe", "Kitapların seni bekliyor."),
        Visitor::new("Mehmet", "Nerelerdeyin? Yeni bir satranç maçına var mısın?"),
    ]
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
