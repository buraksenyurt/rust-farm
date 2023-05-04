/*
   Concurrency uygulanan hallerde state'i thread'ler arasında paylaşmak için kullanılan
   yöntemlerden birisi de shared-memory model'dir(shared-state olarak da geçer)

   Bu modelde kilit noktalar Mutex ve Arc yapılarıdır. Mutext bir veri parçasına sadece tek bir
   thread'in erişebilmesini garanti eder.

   Diğer yandan veriyi bu şekilde kilitlemek yeterli değildir. Verinin sahipliğinin de
   thread'ler arasında taşınabilmesi gerekir. Bu amaçla reference-counted veya atomically reference
   counted olarak adlandırılan smart pointer'lar kullanılır.

   Rc, birden fazla sahip için clone fonksiyonunu kullanarak bu işi çözer ama thread-safe değildir.
   Arc ise thread-safe'tir ve Mutex kurguları içinde idealdir.

   Rc, sahiliğin taşınmasını clone ile çözerken Arc clone operasyonunu farklı şekilde kullanır.
   clone'lama aynı veri kaynağını işaret eden Arc referanslarının örneklemesidir. clone'lama oldukça
   referens sayacı artır, clone scope dışına çıktığında ise azalır. Tabii son clone'da scope dışına
   çıkınca Arc işaretçisi(pointer) ve işaret ettiği değer yok edilir.
*/
use crate::code_stats::CodeStats;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{fs, thread};

mod code_stats;

/*
   Örnek uygulamada kendi bilgisayarımdaki 4 klasör içerisinde yer alan rust dosyalarının
   bazı toplam istatistikleri yakalanmakta. Toplam kod satır sayısı, yorum satırı sayısı,
   boş satırların sayısı gibi...
*/

fn main() {
    let sources = vec![
        "/home/buraks/development/rust-farm/101",
        "/home/buraks/development/rust-farm/handson",
        "/home/buraks/development/rust-farm/Practices",
        "/home/buraks/development/rust-farm/WebDev",
    ];

    let cumulative_stats = CodeStats {
        lines_of_code: 0,
        number_of_files: 0,
        comments_count: 0,
        blank_lines: 0,
        struct_count: 0,
        enum_count: 0,
    };

    // Atomically Reference Counter Mutex koruyucusu ile tanımlanır
    // Korumaya alınan veri modeli CodeStats nesnesidir.
    let stats_arc = Arc::new(Mutex::new(cumulative_stats));

    let mut handlers = vec![];
    // İncelenecek klasör sayısı kadar çalışacan bir döngü
    // Her klasör için ayrı bir thread başlatılır(spawn)
    for source in sources {
        // Arc nesnesinin bir klonu alınır
        let stats = Arc::clone(&stats_arc);
        // ve başlatılan thread'e move sayesinden aktarılır
        let handle = thread::spawn(move || {
            let files = map_files(source);
            calc_files(stats, &files);
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }
    println!("Kod İstatistikleri: {:#?}", stats_arc.lock().unwrap());
}

fn calc_files(stats: Arc<Mutex<CodeStats>>, files: &Vec<DirEntry>) {
    for file in files {
        let content = fs::read_to_string(file.path()).unwrap();
        let mut stats_pointer = stats.lock().unwrap();
        for line in content.lines() {
            if line.is_empty() {
                stats_pointer.blank_lines += 1;
            } else if line.starts_with("//") {
                stats_pointer.comments_count += 1;
            } else {
                stats_pointer.lines_of_code += 1;
                if line.contains("struct") {
                    stats_pointer.struct_count += 1;
                } else if line.contains("enum") {
                    stats_pointer.enum_count += 1;
                }
            }
        }
        stats_pointer.number_of_files += 1;
    }
}

fn map_files(source: &str) -> Vec<DirEntry> {
    let mut dirs = vec![PathBuf::from(source)];
    let mut files = vec![];
    while let Some(dir) = dirs.pop() {
        for dir_entry in fs::read_dir(dir).unwrap() {
            let entry = dir_entry.unwrap();
            if entry.path().is_dir() {
                dirs.push(entry.path());
            } else if entry.path().extension() == Some(OsStr::new("rs")) {
                println!("{:?}", entry);
                files.push(entry);
            }
        }
    }
    files
}
