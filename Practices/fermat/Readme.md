# İlkel Bir Yorumlayıcı Nasıl Yazılır?

Üniversite yıllarından beri eksikliğini en çok hissettiğim şey derleyici ve yorumlayıcıların nasıl çalıştığının anlatıldığı öğretiler olmuştur. Zaman içerisinde hep .Net Framework gibi konularla veri odaklı uygulamar üzerinde çalışınca da bu konu iyiden iyiye unutulmuştur. Yaşım ilerlemiş olmasına rağmen öğrenmek istediğim bir mevzu ve Rust bunun için oldukça ideal. Gerçekten sistem programlama konusunda bir şeyler başarmak istiyorsam derleyici ve yorumlayıcıların çalışma prensiplerini öğrenmeliyim. 

Bireysel düşüncelerimi bir kenara bırakalım ve konumuza dönelim. Kaynaklardan öğrendiğim kadarı ile basit bir yorumlayıcı yazmaya çalışacağız. Öncelikle çok basit bir matematik işlemi düşünelim. Birkaç aritmetik operatörün bir araya geldiği, parantezlerin kullanıldığı, sayılardan oluşan bir ifade tümcesi. Şunu gibi;

```text
4 + (5 * 6) -1
``` 

Komut satırında girilen bu ifadeyi Rust ile nasıl yorumlarız? Yorumlarız derken basit bir parse işleminden bahsetmiyoruz. Belli kurallar çerçevesinde bu ifadeyi alıp parçalarına ayıran ve sonuç üreten bir yapıdan bahsediyoruz. Bir başka deyişle derleyiciler için önem arz eden soyut söz dizimi ağacının _(Abstract Syntax Tree)_ çıkarılıp işletilmesinden... Göründüğü kodar kolay bir iş değil. En azından benim için. Ancak adım adım ilerleyip öğrenmeye çalışacağım.

