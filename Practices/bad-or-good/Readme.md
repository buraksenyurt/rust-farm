# Kötü Kod İyi Kod

Bu kütüphanede birkaç fonksiyon yer alıyor. Bad olarak işaretlenmiş fonksiyonlar sürdürülebilirlik _(Sustainability)_ açısından negatif etkide olanlar. Ayrıca kütüphanedeki fonksiyonlar için benchmark ölçümleri de var. Aslında Green Coding konsepti kapsamında neler yapılabileceğini araştırıyorum. Green Coding prensiplerini aşağıdaki gibi özetleyebiliriz.

- Efficiency (Verimlilik) : Enerji tüketimini daha iyi kod yazarak azaltmaya çalışmanın bir sonucudur. Bu genellikle algoritmaların zaman (Time) ve boyut (Space) karmaşıklığına göre optimize edilmesi anlamına gelir. Yani Big O mühim bir meseledir.
- Kaynak Tüketiminin Azaltılması (Reduciny Resource Consumption) : CPU ve bellek tüketimini minimum seviyeye çeken veri yapılarının ve algoritmaların kullanılmasını öğütler.
- Ölçeklenebilirlik (Scalability) : Enerji tüketimini doğrusal olarak artırmadan sistemleri doğru şekilde tasarlayabilme prensibidir.
- Tasarımda Sürdürülebilir Olmak (Sustainability in Design) : Yazılan kodun uzun vadede nasıl bir etkisi olacağını düşünme prensibidir. Günü kurtaran kod bile olsa bu kodun uzun vadede ektisinin ne olacağını düşünmek gerekir

## Fibonacci Değeri Hesaplaması

Fibonacci yapısında iki fonksiyon yer alıyor. Aslında bir programlama dilinde öz yinelemeli (Recursive) fonksiyonları öğrenirken sık kullanılan problemlerden birisidir Fibonacci hesaplaması. Ve genellikle calc_worst fonksiyonunda olduğu gibi hesaplanır. calc_worst'te her çağrı n-1 ve n-2 için birer çağrı daha yapar. Yani her seviyede fonksiyon çağrılarının sayısı iki katına çıkar. Dolayısıyla Big O değeri açısından bakarsak Time Complexity değeri O(2^n) olur. Her seviyede fonksiyon çağrılarının sayısı iki katına çıktığı için üssel bir büyüme söz konusudur. Dolayısıyla büyük bir sayı için fibonacci hesaplaması istendiğinde işlem uzun sürer. 51 değeri için bunu deneyebilirsiniz. Bu çok doğal olarak işlemcinin daha uzun süre hesaplama yapması, belleğin daha fazla kullanılması anlamına da gelir. Green Code prensipleri için pek de ideal değil. 

Şimdi yazılması biraz daha zor olan ikinci fonksiyona bakalım, calc_green. Bu fonksiyon fibonacci hesaplaması için Memoization tekniğini kullanır. Bu teknikte amaç önceden yapılmış hesaplamaların bir HashMap' te tutulması ve tekrar ihtiyaç duyulması halinde yeniden hesaplamaya gerek kalmadan kullanılabilmesidir. Dolayısıyla her sayı için hesaplama sadece bir kez yapılır diyebiliriz. Her sayı için tek bir hesaplama yapılması zaman karmaşıklığının O(n) olması anlamına gelir. Bu modelin kıymeti özellikle yüksek fibonacci sıra sayıları için anlamlıdır. 51 değerinin hesaplamasına birde bu fonksiyonla bakın derim ;)

