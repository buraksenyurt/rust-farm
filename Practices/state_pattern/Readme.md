# State Tasarım Kalıbının Uygulanması

Belli bir akış içerisinde ele alınan nesneler belli durumlara sahiptirler. Bu durumlar arasındaki geçişler için fonksiyonlardan yararlanılırken bazı kuralların işletilmesi de istenebilir. Örneğin belli bir duruma sahipken diğer bir duruma geçilmesini engelleyen karak mekanizmaları ve koşullar söz konusudur. Nesne yönelimli dillerde bu gibi ihtiyaçlar için State tasarım kalıbı sıklıkla kullanılır. Hatta oyun programlamanın da önemli yapı taşlarından birisi olan State Machine kullanımında da nesne durumlarının yönetimi söz konusudur. Pek tabii nesne yönelimli diller denince işin içerisine interface, class ve object gibi kavramlar da dahil olur. Rust açısından olaya bakınca elimizde struct ve ortak davranış modellemesi için kullanabileceğimi trait'ler vardır. Bu örnekte State tasarım kalıbının Rust tarafında nasıl uygulayabiliriz, örneklerle anlamaya çalışacağız.

Ancak öncesinden bir senaryo düşünelim. Örneğin iş biriminin bir ürünle ilgili yazılım değişiklik taleplerinin belli bir akışı olduğunu düşünelim. Talebin kendisini ChangeRequest gibi bir veri modeli olarak tasarlayabiliriz. Bu nesnenin kullanıldığı akışın da belli kuralları olduğunu düşünelim. Şöyle ki,

- Talepler başlangıçta Draft durumundadır ve boş bir şekilde oluşur.
- Draft tamamlandıktan sonra söz konusu değişiklik talebi gözden geçirilmek üzere bir onaya gönderilir ki bu noktada nesnenin durumu örneğin WaitingForApproval gibi bir konumdadır.
- Talep onaylandığı takdirde yazılım ekibinin backlog'una önceliklendirilmek üzere girebilir. Bu noktada talep örneğin Committed gibi bir konumda olabilir.
- Sadece onaylanan yazılım değişiklik talepleri backlog'a akar. Bir başka deyişle onaylanmayan yazılım değişiklik talepleri hiçbir şekilde Committed durumuna geçememelidir. 
- Benzer şekilde henüz Draft olarak dünyaya gelen bir yazılım değişiklik talebi, gözden geçirilme ve onay mekanizmasını atlayarak Committed durumuna geçemez. Bir başka deyişle gözden geçirme durumu için bir geçiş işlemi uygulanmamışsa Draft konumunda kalması gerekir.

Benzer şekilde farklı senaryoları da düşünebiliriz. Örneğin bir şirketin veritabanında personelin açtığı ve birtakım onay mekanizmaları sonrası işletilen SQL betiklerine ait akışta da State tasarım kalıbı pekala kullanılabilir. Ya da bir terminal oyununun menüsünden oyunun oynandığı duruma geçmek, oyuncu yandığında oyunu tekrardan başlatmak üzere başlangıç konumuna döndürmek gibi durumların ele alınmasında da bu kalıbı pekala kullanabiliriz.

İlk senaryomuza geri dönelim ve adım adım kodlamasını yapmaya çalışalım. Yazılım değişiklik talebinin kendisini ChangeRequest isimli bir struct olarak tanımlayabiliriz. State olarak ifade edilen hallerini ise Draft, WaitingForApproval ve Committed şeklinde üç struct'la ifade edebiliriz. Durumlar arasındaki geçişleri sağlamak içinse ortak davranışlar tanımlamamızı sağlayacak bir trait'ten faydalanabiliriz.

Örneğimizi bir kütüphane olarak açarak devam edelim.

```shell
cargo new --lib state_pattern
```

İlk olarak aşağıdaki şekilde ilerleyelim.

```rust

```