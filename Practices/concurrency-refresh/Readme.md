# Concurrency

Notlar

- Kodun belli parçalarının ayrı thread'lerde ve paralel olarak çalışabilmesidir.
- Eş zamanlı çalışması istenen kod parçalarına ait thread'leri açmak için thread::spawn fonksiyonu kullanılır.
- Tüm thread'lerin bitmesini garanti etmek için bu thread'leri taşıyan handle nesneleri ve join fonksiyonu kullanılır.
- Thread dışındaki bir değerin sahipliğini içeriye almak için move closure'ı kullanılır.
- Thread'ler arasında mesaj taşımak için channel nesnesi kullanılır. Transmitter ile mesaj yollanır, receiver ile mesaj okunur.
- Bir kanal, generic Sender nesnesi ile fonksiyona geçilebilir.
- Bir kanala n sayıda mesaj gönderilebilir. Döngü ile veya tekere teker. Bu değerleri okurken receiver üstünden döngü kurulabilir. Her iterasayon bir mesajın dönüşünü ifade eder.
- Birden fazla thread üstünden kanala mesaj bırakmak için transmitter nesnesini klonlarız.
- Thread'ler arasında bellek paylaşımı için Arc nesnesinden yararlanılır. Data Races oluşmasını engellemek için yaygın yöntem Mutex ve Arc nesnelerinin birlikte kullanımıdır. 
- Mutex, veriyi kitleyip data races durumunun oluşmasını engellemek için kullanılır.
- Arc belleği ele alırken Mutex veriye odaklanır.
- Ortak veriye her erişip değiştirmek istediğimzde Mutex ile kitlememiz gerekir.