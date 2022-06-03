/*
    Başlangıçta çalışacak olan bu dosya index.js gibi modüllerin yüklenmesi için kullanılabilir.
    Modüller yüklenirken oluşabilecek hatalar burada ele alınabilir.
*/

import("./index.js")
    .catch(e => console.error("Index modülün yüklenirken hata.",e))