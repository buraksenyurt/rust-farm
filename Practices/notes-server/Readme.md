# Notes Server

Bu uygulama sadece index.html sayfasını host eden basit bir web sunucu uygulamasıdır. Sayfa her talep edildiğinde benim kitaplardan, bültenlerden, dergilerden derlediğim notlardan rastgele birisini göstermektedir. 

Örnek projede warp, tokio, serde, handlebars küfeleri kullanılmakta. warp web sunucusu, tokio asenkron operasyonlar, serde json serileştirme operasyonları ve handlebars' da HTML şablonundaki dinamik içerikleri yönetmek için kullanılmaktadır.