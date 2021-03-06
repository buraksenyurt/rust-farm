# Bracket-Lib ile Bir Packman Oyunu Denemesi

Herbert Wolverson'un yazdığı [Hands-on Rust: Effective Learning through 2D Game Development and Play](https://www.amazon.com/Hands-Rust-Effective-Learning-Development-dp-1680508164/dp/1680508164/) kitabından öğrendiklerimle popüler Packman oyununun bir benzerini yazmaya çalışmaktayım. Yol boyunca rust dilini öğrenmek adına bana kattığı pek çok şey oluyor.

#### Sürüm 0.1.1

- Sahneye yer, duvar, elma ve çürük elma ile packy karakteri başarılı bir şekilde yerleştirildi.
- Tam 10 yenebilir elma ve 5 çürük elma eklendi.
- Açılışta Duvar, Elma, Çürük Elma ve Packy sadece boş olan elverişli alanlara yerleşebiliyor.
- Her açılışta Packy müsait olan rastgele bir konumda oyuna başlıyor.
- Packy duvarlardan geçemez *(Şimdilik. Sonrasında hak sayısına göre duvar patlatabilir)*
- Pack elma ve çürük elmalar üstünden şimdilik geçebiliyor ama sonraki sürümde sağlıklı elmalardan artı puan, çürük elmalardan eksi puan alacak ve geçtiği karelerdeki elmalar sahneden çıkarılacak.

![../images/packman_1.png](../images/packman_1.png)

#### Sürüm 0.1.2

- Packy, sağlam elamları yediğinde 10 puan, çürük elma yediğinde -5 puan alıyor.
- Skor tabelası eklendi.
- 3 bomba hakkı verildi. Packy, space tuşu ile bomba patlatabiliyor. Bu durumda üst, sol, sağ ve alt kısımlarda duvar varsa patlıyor.
- Apple ve Rotten_Apple veri yapıları tek veri yapısında toplandı.
- Oyuna giriş menüsü eklendi.
- Warp (W) ile rastgele bir konuma atlama özelliği eklendi.
- Hayalet karakteri eklendi. Şimdilik boş boş dolanıyor.

Bomba ile duvarları patlatma özelliği için aşağıdaki çizelgeden yararlanıldı.

![../images/packman_2.png](../images/packman_2.png)

Giriş menüsüne ait bir görüntü.

![../images/packman_3.png](../images/packman_3.png)

Ghost eklendikten sonraki duruma ait bir görüntü.

![../images/packman_4.png](../images/packman_4.png)

#### Sürüm 0.2.0

- Hayaletin hızını ayarlayabilmek için options(O) menüsü eklendi.
- Hayalet, kırmızı elma yediğinde rakibin puanından 3 birim azalması sağlandı.
- Packyman ve hayaletin bazı hallerde dört tarafı duvarla örülü şekilde kalması engellendi.
- Hayalet packyman'i yakaladığında oyunun sonlanmasının sağlanması.
- Packyman'in skor tabelasına çıkması engellendi.
- x=0 sütunu oyuna dahil edildi.
- Packyman'in kırmızı elmaları toplama süresinin ölçülmesi sağlandı.
- Kazanma ve kaybetme modları eklendi.
- Duvar, elma ve çürük elma simgeleri elle yeniden tasarlandı.

![../images/packman_5.png](../images/packman_5.png)

![../images/packman_6.png](../images/packman_6.png)

![../images/packman_7.png](../images/packman_7.png)

#### Sürüm 0.2.1

- Hayaletin hareket mekanizması packyman'in konumuna göre yeniden programlandı. 
- Hayalet duvarlardan geçebiliyor :D
- Oyuna rastgele hareket eden yeni bir hayalet nesnesi eklendir.

*Planlananlar*

- Oyuncu space ile bomba patlattığında veya W ile ışınlandığında bu ekranda bilgi olarak çıksın.
- Hayalet rastgele bomba patlatabilsin.