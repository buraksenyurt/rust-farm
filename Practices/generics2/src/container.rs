/*
   Generic kullanımında kendi kısıtlarımızı koymamız gerekebilir.
   Özellikle generic türü belli davranışları içermeye zorladığımızda, çalışma zamanının
   bu kurallara uyan nesneleri ele alması garanti edilir.
   Bunun için trait enstrümanından yararlanılır.
*/
pub trait Container<T> {
    fn get(&mut self) -> Option<T>;
    fn push(&mut self, item: T);
    fn is_empty(&self) -> bool;
}
