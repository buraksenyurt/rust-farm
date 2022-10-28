//noinspection RsMainFunctionNotFound
mod song;

#[macro_use]
extern crate rocket;

use crate::song::Song;
use rocket::{Build, Rocket};
use std::collections::LinkedList;
use std::sync::{Mutex, MutexGuard};

// İstek şarkıları tutan kuyruk. Bir Mutex nesnesi olarak kullanılıyor.
// Nitekim API'yi kullanan n sayıda client için bu liste paylaşımlı bir veri
static SONG_QUEUE: Mutex<LinkedList<Song>> = Mutex::new(LinkedList::new());

#[launch]
fn rocket() -> Rocket<Build> {
    Rocket::build().mount("/", routes![])
}

fn get_queue<'a>() -> MutexGuard<'a, LinkedList<Song>> {
    SONG_QUEUE.lock().expect("Mutex ile bir sorun yaşanıyor")
}
