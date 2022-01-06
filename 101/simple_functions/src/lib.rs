use std::f32::consts::PI;

#[cfg(test)]
mod tests {
    use crate::{greetings, magic, move_forward, square_of_circle};

    #[test]
    fn should_greetings_works_test() {
        let name = "Wonder Woman";
        let result = greetings(name);
        assert_eq!(result, "Merhaba Wonder Woman.");
    }

    #[test]
    fn should_10_returns_314_15927_test() {
        let r = 10.0;
        let result = square_of_circle(r);
        assert_eq!(result, 314.15927);
    }

    #[test]
    fn should_move_forward_works_test() {
        let (a, b) = move_forward(5, 10);
        assert_eq!(a, 15);
        assert_eq!(b, 20);
    }

    #[test]
    fn should_magic_do_magic_test() {
        let mut info = "Güzel bir gün";
        magic(&mut info);
        assert_eq!(info, "Heh heh heee :P");
    }
}

pub fn greetings(your_name: &str) -> String {
    let message = format!("Merhaba {}.", your_name);
    message
}

pub fn square_of_circle(r: f32) -> f32 {
    PI * r * r
}

pub fn move_forward(x: i32, y: i32) -> (i32, i32) {
    (x + 10, y + 10)
}

pub fn magic(messsage: &mut &str) {
    println!("Gelen mesaj...\n{}", messsage);
    *messsage = "Heh heh heee :P";
}
