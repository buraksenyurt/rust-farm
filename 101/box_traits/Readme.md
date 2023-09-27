# Box Kullanımı

Box, smart pointer türlerinden birisidir ve veriyi Heap'te saklamak için bir yol sunar. Genellikle compile-time'da yaşam ömürleri bilinmeyen veya dinamik olarak değişebilen verileri Heap alanında saklamak için kullanılır. Örneğin stack bellek bölgesinde tutulan bir i32 değeri heap'a Box ile alınabilir. Trait'lerle birlikte kullanılabilir.

## Avantajları

- Verinin boyutu veya ömrü compile-time'da bilinmeyen durumlarda çok kullanışlıdır.
- Lifetime analizini basitleştirir çünkü veriler heap bölgesinde saklandığından lifetime sorunu olması olasılığı azdır.
- Trait'ler ile birlikte kullanıldığında polimorfik davranışları destekler Box< dyn Trait > kullanımı.
- Farklı veri türlerinin aynı veri yapısında(örneğin bir koleksiyon) saklanmasını sağlar.
- Tüm veriyi kopyalamak yerine yanlızca işaretçinin kopyalanmasını sağladığından büyük veri yapılarının taşınmasında esneklik sağlar.
- Verinin mülkiyetini ve sahipliğini açıkça belirtmemizi sağlar.

## Dezavantajları

- Heap kullanımı söz konusu olduğundan performans dezavantajı vardır.
- Verinin ne zaman serbest bırakılması gerektiğini kontrol etmek gerekir nitekim bellek sızıntıları olabilir.
- Verinin box içine alınması ekstra işlem karmaşıklığına yol açar.

## Smart Pointer Demişken

Rust dilinde birkaç farklı Smart Pointer vardır. Her biri farklı senaryolarda işe yarar.

- **Reference Counted(Rc):** Bir verinin birden fazla sahibi olabileceği durumlarda sahiplik sayısının izlenmesi için kullanışlıdır. Döngüsel bağımlılık sorunlarını engellemeyi kolaylaştırır.
- **Atomic Reference Counted(Arc):** Çoklu thread yapılarından verinin güvenli şekilde paylaşımlı olarak kullanılmasında ele alınır.
- **Mutex ve RwLock:** Paylaşılan veriye erişimi senkronize etmek için kullanılırlar. Mutex veriyi tek bir thread'in kullanmasını garantiler. RwLock ise Read-Write Lock anlamına gelir ve verinin birden fazla iş parçacığınca okunmasına izin verirken yazma işlemlerini senkronize etmekte kullanılır.
- **Cell ve RefCell:** Cell ile verinin değiştirilmesi engellenir. Verinin mutably korumak istediğimiz durumlarda ele alabiliriz. RefCell ise veriyi çalışma zamanında güvenli şekilde mutably olarak ödünç almamıza olanak tanır.

Rc ve Arc paylaşımlı veri sahipliği olan senaryolar için idealken, Mutex ve RwLock çoklu iş parçacığı senaryoları için daha uygundur. Box ise daha basit ve tek sahipli verileri saklamak için kullanılır.