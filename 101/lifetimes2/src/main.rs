fn main() {
    let bruce = Actor::new(48, String::from("Bruce Wayne"));
    println!("({}){}", bruce.sys_id, bruce.get_name());
}

// Birde lifetime bilgisinin otomatik olarak atandığı haller vardır.
#[derive(Debug)]
struct Actor {
    sys_id: u16,
    name: String,
}
impl Actor {
    fn new(sys_id: u16, name: String) -> Self {
        Actor { sys_id, name }
    }
    // Geriye String nesne referansı döndüren get_name metodunu
    // lifetime parametresini açık bir şekilde belirterek aşağıdaki gibi yazdığımızı düşünelim.
    // Kod çalışacaktır ancak aşağıdaki gibi açıkça yaşam ömrünü belirtmemiz gerek yoktur.
    // explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
    fn get_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}
