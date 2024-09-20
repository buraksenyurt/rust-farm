# Ownership için Farklı Bir Bakış Açısı

Rust dilinde önemli yetkinlik seviyelerinden birisi OBL'i, yani Ownership, Borrowing ve Lifetimes üçlemesini iyi şekilde öğrenmektir. Ownership için sıklıkla kullanılan örneklerden birisi aynı değeri referans eden nesneler üzerindeki değişikliklerin birbirlerini etkilemesidir. Rust, Ownership ilkesinde temel olarak referans vermeyi ve veriyi değiştirmeyi mümkün mertebe sınırlandırmaya çalışır.

Bu örnek projede basit bir Javascript kodu yer alıyor. Bu kodun çalışma zamanı çıktısı aslında bir bug olduğunu göstermekte ancak kolaylıkla gözden kaçabilecek bir durumu da ifade etmekte.

Js kodumuz kabaca aşağıdaki gibi.

```js
const health = {
    value:100
};

const archer = {
    id : 1,
    name : "legolas",
    current_health : health
}

const orc = {
    id:2,
    name : "rando",
    current_health: health
}

function damage_to_orc(player,value){
    if(player.name === 'rando') {
        player.current_health.value -= value;
    }
}

console.log("Orc health before:", orc.current_health.value);
damage_to_orc(orc, 10);
console.log("Orc health after:", orc.current_health.value);
// Esasında damage_to_orc fonksiyonunda sadece 'rando' için bir damage söz konusu
// Ne var ki
console.log("Legolas health afrer: Upss",archer.current_health.value);
```

Sistemde NodeJs yüklü ise aşağıdaki komutla kolayca çalıştırabiliriz.

```bash
node gameController.js
```

Durumu analiz edelim. archer ve orc isimli nesneler health isimli nesneyi kullanıyorlar. Her ne kadar damage_to_arc fonksiyonuna sadece orc nesnesini göndersek de ve içeride name alanını kontrol edip değer değişikliğini sadece rando için yapsak bile, hem archer hem orc değişkenleri aynı health nesnesini referans ettiğinden birisinde yapılan değişiklik diğerini de etkliyor. Dolayısıyla sadece rando'nun health değerini değiştirdiğimizi düşünürken esasında legolas'ın health değerinin de değişmiş olduğunu görüyoruz.

```text
node .\gameController.js

Orc health before: 100
Orc health after: 90
Legolas health afrer: Upss 90
```