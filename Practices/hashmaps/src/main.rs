use std::collections::HashMap;

/*
   HashMap ile key:value türünden değerler tutulabilir.
   Key kullanılabilmesi için bazı trait'lerin implemente edilmiş olması beklenir.
   Eq, PartialEq ve Hash
*/
fn main() {
    let mut code_map = HashMap::new();

    code_map.insert(Codes::ALPHA, vec![1, 6, 1, 7]);
    code_map.insert(Codes::FOX, vec![6, 3, 8, 2]);
    code_map.insert(Codes::SIERRA, vec![5, 5, 0, 1]);
    // Aşağıdaki gibi bir for döngüsü ile (key:value) çiftlerini dolaşabiliriz
    for (k, v) in code_map.iter() {
        println!("{:?}\t{:?}", k, v);
    }
    // Aynı işi consumer fonksiyonları kullanarak da yapabiliriz.
    code_map.iter().for_each(|(k, v)| {
        println!("{:?}\t{:?}", k, v);
    });

    // code_map listesinde SIERRA key değerinin sahipliğini alıyoruz.
    // or_insert kullanarak, SIERRA bulunmadığında varsayışan bir vector ile onun
    // hashmap'e eklenmesini de sağlayabiliriz. Yani eklediğimizin sahipliğini alabiliriz.
    let sierra_codes = code_map.entry(Codes::SIERRA).or_insert(vec![0, 0, 0, 0]);
    println!("Sierra codes: {:?}", sierra_codes);

    let gamma_codes = code_map.entry(Codes::GAMMA).or_insert(vec![0, 0, 0, 0]);
    println!("Gamma codes: {:?}", gamma_codes);

    // get fonksiyon çağrısı Option döndüğünden if let ifadesi ile de ele alınabilir
    if let Some(alpha_codes) = code_map.get(&Codes::ALPHA) {
        println!("Alpha codes: {:?}", alpha_codes);
    }

    // ya da aşağıdaki gibi de yazılabilir
    if code_map.contains_key(&Codes::FOX) {
        println!("Fox codes: {:?}", code_map.get(&Codes::FOX).unwrap());
    }

    // ALPHA'yı çıkartıp kalanları bırakır
    code_map.retain(|k, _| k != &Codes::ALPHA);
    println!("Code map: {:#?}", code_map);
}

// Codes enum sabiti key olabilme koşullarını sağlıyor (
#[derive(Debug, Eq, PartialEq, Hash)]
enum Codes {
    ALPHA,
    LIMA,
    SIERRA,
    FOX,
    GAMMA,
    BETA,
}
