/*
   Abstract Syntax Tree üstündeki boğumları Node isimli aşağıdaki enum sabiti ile ifade edebiliriz.
   Toplama, Çıkartma, Çarpma, Bölme ve Üst alma işlemlerinde iki taraf vardır.
   Ağaç yapısını oluşturan fonksiyonda bu enum sabitlerinden yararlanarak işlem sonucunu bulmayı planlıyoruz.
   Çok doğal olarak node'lar kendi alt node'larını barındıracağından her öğe birer Node taşır.(Number hariç)

   Box kullanılmasının sebebi ise Node içeriklerini Heap üstünde saklamaktır.
*/
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Substract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}
