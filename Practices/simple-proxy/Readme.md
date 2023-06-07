# Basit Bir Reverse Proxy Senaryosu

Bu örnekte amaç çok basit seviyede bir Reverse Proxy Server geliştirmek. Proxy server'lar aracı yazılım servisleri olarak internet üzerindeki çoklu ağlar arası geçişlerde yön bulma sağlarlar. İki çeşit proxy server vardır. Forward Proxy ve Reverse Proxy.

Forward proxy, istemcilerin internete doğru yaptığı talepler için gateway rolü üstlenirler. İstemci makinelerin IP adresleri internette gezinirken gizli kalır nitekim internete çıkışı Forward Proxy Server sağlar. Böylece internete çıkan istemciler için organizasyonel politikaların uygulanması mümkün hale gelir. Mesela bazı adreslere çıkışlar kısıtlanabilir.

Bizim örneğimizde ele alacağımız konu ise Reverse Proxy. Bu sefer backend server'ların kimliği istemciden gizlenir. İstemci talepleri backend server'lara geçirilirken load balancing, caching, compression gibi çeşitli fonksiyonellikler kullanılabilir. Backend server'larda işlenen taleplere ait cevaplarda yine Proxy Server üstünden istemcilere iletilir.

![../images/simple_proxy_01.png](../images/simple_proxy_01.png)

Senaryo da iki adet program söz konusu olacak. Birisi backend server görevini üstlenirken diğeri reverse proxy server olacak.

Origin Server'ın görevleri arasında HTTP taleplerini almak, talebin ilk satırını ayrıştırıp GET metodunu ve belli bir route bilgisini yakalamak, sonrasında da bir cevap dönmek yer alıyor.

## Çalışma Zamanı