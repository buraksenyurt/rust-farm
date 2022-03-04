## ~~# İlkel Bir Yorumlayıcı Nasıl Yazılır. İPTAL?~~

Üniversite yıllarından beri eksikliğini en çok hissettiğim şey derleyici ve yorumlayıcıların nasıl çalıştığının anlatıldığı öğretiler olmuştur. Zaman içerisinde hep .Net Framework gibi konularla veri odaklı uygulamar üzerinde çalışınca da bu konu iyiden iyiye unutulmuştur. Yaşım ilerlemiş olmasına rağmen öğrenmek istediğim bir mevzu ve Rust bunun için oldukça ideal. Gerçekten sistem programlama konusunda bir şeyler başarmak istiyorsam derleyici ve yorumlayıcıların çalışma prensiplerini öğrenmeliyim. 

Bireysel düşüncelerimi bir kenara bırakalım ve konumuza dönelim. Kaynaklardan öğrendiğim kadarı ile basit bir yorumlayıcı yazmaya çalışacağız. Öncelikle çok basit bir matematik işlemi düşünelim. Birkaç aritmetik operatörün bir araya geldiği, parantezlerin kullanıldığı, sayılardan oluşan bir ifade tümcesi. Şunun gibi;

```text
4 + (5 * 6) -1
``` 

Komut satırından girilen bu ifadeyi Rust ile nasıl yorumlarız? Yorumlarız derken basit bir parse işleminden bahsetmiyoruz. Belli kurallar çerçevesinde bu ifadeyi alıp parçalarına ayıran ve sonuç üreten bir yapıdan bahsediyoruz. Bir başka deyişle derleyiciler için önem arz eden soyut söz dizimi ağacının _(Abstract Syntax Tree)_ çıkarılıp işletilmesinden... Göründüğü kadar kolay bir iş değil. En azından benim için. Ancak adım adım ilerleyip öğrenmeye çalışacağım.

İlk olarak belli başlı kuralları ve kavramları konuşmakta yarar var. Yukarıdaki matematiksel ifade doğru görünüyor. Kullanıcının girdiği buna benzer bir ifadenin doğru olması için cümlede aritmetik operatörler, sayılar ve parantezlerden farklı şeylerin yer almaması lazım. Hatta noktalı sayılarda geçerli kabul edilmeli. Diğer yandan işlem önceliği kuralları burada da geçerli olmalı. Yani paranetez olmadığı hallerde öne çıkan çarpma/bölme işlemleri veya parantez olduğu durumlarda parantezler arasındaki ifadelerin ilk sıraya alınması gibi. Kullanıcı bu ifadeyi serbest formatta komut satırından girecek ve belki bazı yerlerde boşluk karakteri de kullanacak. Buna izin verebiliriz elbette. Ya da çok daha sıkı kurallar düşünüp izin vermeyiz. Bu biraz da case-sensitive bir söz dizimi kuralı uygulatmak gibi bir karar aslında. 

Peki diyelim ki söz konusu ifade kurallara uyuyor. Nasıl bir süreç işletmeliyiz? Temel anlamda akışı aşağıdaki gibi düşünebiliriz.

```text
İfadeyi tara -> Tokenizer ile token listesini çıkar -> Token'lardan Abstract Syntax Tree'yi üret -> Ağaç yapısını işlet _(Evaluator)_
```

Burada çok önemli kavramlar var. İfade içerisindeki sayılar, aritmetik operatörler, parantezler, noktalı sayı işaretleri vs birer token olarak ifade edilirler. Bu token'lar Lexer ya da Tokenizer modülleri tarafından oluşturulur. Lexer'ın görevi metin içindeki anahtar kelimeleri, ifadeleri, operatörleri, fonksiyon çağrılarını anlamlandırıp token olarak açmaktır. Bunu yaptığımız da ifadenin program kodu için anlamlandırılmış karşılıklarını buluruz. Token'lar çıkartıldıktan sonra bunlardan yararlanarak bir söz dizimi ağacı oluşturulur ki bu aşama genellike _parsing_ süreci olarak da bilinir. Parser tarafından oluşturulan bu ağaçta önceliklere göre bir sıralama söz konusudur. Bir başka deyişle işletilecek sürecin kurallarına göre bir ağaç yapısı tesis edilir. Bu örnek özelinde matematik önceliklerine göre inşa edilmelidir. Ağaç yapısındaki node'lar istenen doğru sırlamaya geldikten sonra yorumlayıcı _(yürütücü)_ bu node'ları toplayarak _(ki aggregate safhası olarak ifade edilir)_ bir sonuç bulur. Yani girilen matematiksel dizilimin sonucu :) 

Şimdi herhangibir script dili ifadesini düşünün. Mesela bir SQL cümleciğini ya da terminalden girilmiş bir Python kod parçasını. Yorumlayıcı için gerekli temel çalışma prensipleri yukarıda bahsettiğimiz gibidir. Abstract Syntax Tree yapısının ortaya çıkması başka şeyleri de kolaylaştırır. Söz gelimi ortaya çıkan token'ların farklı bir dildeki karşılıkları ile değiştirilmesi başka bir dile çevrilmeyi pekala mümkün kılabilir. C# ile yazılmış bir kod parçasının alınıp IL kodlarına çevrilmesi belki bu felsefede düşünülüp yorumlanabilir.

Gelelim örneğimize...Pek tabi işe projeyi ve gereki dosyaları oluşturarak başlamakta yarar var.

```shell
cargo new fermat
cd fermat
mkdir src/parser
touch src/parser/mod.rs
touch src/parser/token.rs
touch src/parser/lexer.rs
touch src/parser/tree.rs
touch src/parser/parser.rs
touch src/parser/node.rs
```

İşlerin kolay takibi için kavramlar ayrı modüller içerisinde toplanmış durumda.

- Önce token veri modeli ve bağlı fonksiyonları geliştirilir.
- Takiben lexer modülü yazılır. Bu yapı token'ları keşfeder.
- 

```shell
# Örneği çalıştırmadan önce
cargo clippy
# testleri koşturmak için
cargo test
```

_**Kaynak: Practical System Programming for Rust Developers, Prabhu Eshwarla, Packt Publishing**_