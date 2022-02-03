# Basit Seviyede Birkaç Problem ile Uğraşıyoruz

Bu örnekte birkaç problemi çözmeye çalışıyoruz. Bu problemleri şöyle sıralayabiliriz.

- [x] Bir sayının __armstrong__ sayısı olup olmadığını bulan fonksiyon.
- [x] __Collatz__ sayı teroisini uygulayan bir fonksiyon.
- [x] N adet doğal sayının toplamlarının karesi ile karelerinin toplamı arasındaki farkı bulan bir fonksiyon.
- [ ] Bir RNA dizilimini proteinlere dönüştüren fonksiyonu.
- [ ] __Affine Cipher(Doğrusal Şifreleme)__ yani doğrusal şifreleme yapan bir fonksiyon.

Şimdi haberler :P

## Armstrong Sayıları

Bir sayının rakamlarını alıyoruz ve her birinin rakam sayısı kadar üstünü birbirleriyle topluyoruz. Çıkan toplam sayıya eşitse bu sayı Armstrong sayısıdır. Karışık oldu değil mi :) Örnekleyelim;

9 bir armstrong sayısıdır nitekim 9^1=9 yani kendisidir.
371'i ele alalım. 3^3 + 7^3 + 1^3 = 27 + 343 + 1 = 371 yani kendisidir.
154 ise armstrong sayısı değildir. Nitekim 1^3 + 5^3 + 4^3 = 1 + 125 + 48 = 190 yani kendisine eşit değildir.

## Collatz Konjektürü

Matematikçi Collatz'ın bu teorisinde herhangibir pozitif sayıyı alıyor. Sayı çift ise 2'ye bölüyoruz. Eğer tek sayı ise 3 ile çarpıyor ve 1 ekliyoruz. Doğal olarak yeni bir sayı elde ediyoruz. Oluşan yeni sayıyı yine çift veya tek olmasına göre aynı işlemden geçiriyoruz. Teoriye göre oluşan sayı serisi eninde sonunda 4,2,1,4 şeklinde bir yere varıyor. Hemen bir örnekle açıklayalım.

```text
n = 12 çift sayıdır.
12 / 2      =   6
6 / 2       =   3
3 * 3 + 1   =   10
10 / 2      =   5
5 * 3 + 1   =   16
16 / 2      =   8
8 / 2       =   4
4 / 2       =   2
2 / 1       =   1    
1 * 3 + 1   =   4
4 / 2       =   2  
2 / 2       =   1
1 * 3 + 1   =   4
```

İstenen çözümde ilk 1 rakamına kaç hamlede ulaşıldığının hesaplanması isteniyor. Aklıma ilk gelen yol for döngüleri kullanmak. Lakin daha idiomatic yollar var. Pattern matching içeren recursive fonksiyonlar ve hatta Iterator trait'ini baz alarak count işlevinden yararlananlar.

## Karelerin Farkı

Bu problem aslında [Project Euler sitesinde](https://projecteuler.net/problem=6) yer alan sorulardan birisi. N adet sayının toplamlarının karesi ile her bir sayının karelerinin toplamları arasındaki farkın bulunması isteniyor. Doğal olarak envayi çeşit yol olduğunu söylemek mümkün. Basit for döngülerinden tutun, matematiksel denklemleri kullanarak ya da Higher Order Function'ları işin içine katarak ilerleyebiliriz. 

_Önce biraz teori;_

1'den 10 kadar olan sayıları ele alalım.

Problemin birinci kısmında sayıların karelerinin toplamını bulalım.

**1^2 + 2^2 + 3^2 .... 10^2**

,şeklinde ifade edebiliriz.

Yani **1+4+9+16+...100** gibi bir seri oluşmakta. Bu işlemle ulaşılan değer [__Dörtyüzsel Sayı__](https://tr.wikipedia.org/wiki/D%C3%B6rty%C3%BCzl%C3%BCsel_say%C4%B1) olarak da ifade ediliyor ve pek tabii bir formülü var. 

**sum1 = n(n + 1)(2n + 1)/6**

Problemin ikinci aktöründe ise N sayının toplamlarının karesi isteniyor.

**(1 + 2 + 3 + 4 + ... n)^2** 

,şeklinde düşünebiliriz.

Aslında Gaussça düşünürsek 1den N'e kadar olan sayıların toplamının karesini almamız yeterli. Yani,

**sum2 = (n(n+1)/2)^2**

kod tarafında for döngülü, matematiksel formüllü ve HOFs versiyonlarını kurgulayabiliriz.

İstenen şeyse bu ikisi arasındaki farkın bulunmasıdır.

**result = sum2 - sum1**

Örneğin 10'a kadar olan sayılar için işlem sonucu 2640 olmalıdır.

## RNA'lardan Proteinlere

__todo()!__

## Doğrusal Şifreleme

__todo()!__
