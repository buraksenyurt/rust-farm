use std::fmt::{Display, Formatter};

// Kullanıcı rollerini tutan enum sabiti
#[derive(Clone, PartialEq)]
pub enum Role {
    Admin,
    User,
}

// Bir string veriden Role türüne dönüştürmek için aşağıdaki fonksiyonu kullanabiliriz
impl Role {
    pub fn from_str(r: &str) -> Role {
        match r.to_lowercase().as_str() {
            "admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

// to_string operasyonlarındaki dönüştürme işi için Display trait'ini implemente ederiz
impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::User => write!(f, "User"),
        }
    }
}
