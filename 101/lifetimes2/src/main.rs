fn main() {
    let bruce = Actor::new(48, String::from("Bruce Wayne"));
    println!("({}){}", bruce.sys_id, bruce.get_name());

    /*
        #2
        Dikkat edilmesi gereken bir durum scope'lar söz konusu olduğunda kendini gösterir.
        actors_name isimli değşkeni göz önüne alalım.
        main scope'u içinde String referans olarak tanımlanmış,
        iç scope'ta jennifer değişkeninin name değerini ödünç almış ve
        iç scope sonlandıktan sonra kullanılmak istenmiştir.
        Aslında actors_name'in yaşam süresi, iç scope sonrasına yetmez :D

        borrowed value does not live long enough

    */
    let actors_name: &String;
    {
        let jennifer = Actor::new(32, String::from("Jenifır Lovrıns"));
        println!("{:#?}", jennifer);
        //actors_name = jennifer.get_name();
        actors_name = bruce.get_name();
        // Üstteki atamada ise bir sorun olmaz.Nitekim actors_name ile bruce aynı scope seviyesindedir.
    }
    // Hatayı alabilmek için actors_name'i bilerek scope dışında kullandık.
    println!("{}", actors_name);
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
    /*
        #1
        Geriye String nesne referansı döndüren get_name metodunu
        lifetime parametresini açık bir şekilde belirterek aşağıdaki gibi yazdığımızı düşünelim.
        Kod çalışacaktır ancak aşağıdaki gibi açıkça yaşam ömrünü belirtmemiz gerek yoktur.

        explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
    */
    // fn get_name<'a>(&'a self) -> &'a String {
    //     &self.name
    // }

    fn get_name(&self) -> &String {
        &self.name
    }
}
