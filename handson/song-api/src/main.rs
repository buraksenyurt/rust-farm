/*
   Bir istek parça listesi için Restful API hizmeti vermek istediğimiz düşünelim.
   İstek listesini tutacak veri yapısının bir kuyruk olarak ele alınması gerekir.
   Veri içeriği tüm istemciler için paylaşımlıdır.
   Eş zamanlı veri erişimini kontrol altına almak için Mutex yapısından yararlanılabilir.

   Tutorial kaynağımız : https://imajindevon.hashnode.dev/rust-rocket-song-request-api

   Post örneği

   curl -X POST http://127.0.0.1:8000/add -H Content-Type: "application/json" -d
   '{"title": "Domates Biber Patlıcan","artist": "Barış Manço","album": "Mançoloji II"}'
*/
mod song;

#[macro_use]
extern crate rocket;

use crate::song::Song;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use std::collections::LinkedList;
use std::sync::{Mutex, MutexGuard};

// İstek şarkıları tutan kuyruk. Bir Mutex nesnesi olarak kullanılıyor.
// Nitekim API'yi kullanan n sayıda client için bu liste paylaşımlı bir veri
static SONG_QUEUE: Mutex<LinkedList<Song>> = Mutex::new(LinkedList::new());

#[launch]
fn rocket() -> Rocket<Build> {
    Rocket::build().mount("/", routes![add_song])
}

fn get_queue<'a>() -> MutexGuard<'a, LinkedList<Song>> {
    SONG_QUEUE.lock().expect("Mutex ile bir sorun yaşanıyor")
}

#[post("/add", data = "<s>")]
fn add_song(s: Json<Song>) -> String {
    let mut locked_object = get_queue();
    locked_object.push_back(s.0.clone());
    format!(
        "[{}] isimli şarkı {}. sıraya eklendi.",
        s.title,
        locked_object.len()
    )
}
