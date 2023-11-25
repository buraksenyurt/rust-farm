use crate::formatter::Serializer;
use crate::issue::Issue;
use crate::response::{HttpResponse, Response};
use std::io::Write;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Utility {}

impl Utility {
    pub fn send_response(stream: &mut TcpStream, content: String, res: HttpResponse) {
        let response = Response::new(res, content);
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
    pub fn vec_to_json(issues: Vec<Issue>) -> String {
        let mut json_array = String::from("[\n");
        for (i, issue) in issues.iter().enumerate() {
            json_array.push_str(&issue.to_json());
            if i < issues.len() - 1 {
                json_array.push_str(",\n");
            }
        }
        json_array.push(']');
        json_array
    }

    pub fn gen_guid() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Zaman damgası oluşturmada sorun var.")
            .as_nanos();

        let random_number = timestamp % 1_000_000_000_000_000_000_000;
        /*
           {:08x} sayıyı 8 karakter genişliğinde ondalık formata çevirmek için kullanılır.
           {:04x} benzer şekilde sayıyı 4 karakter genişliğinde ondalık formata çevirmek için.
           {:12x} ise sayısı 12 karakter genişliğinde ondalık formata çevirmek için.

           >> bit seviyesinde kaydırma operatörüdür.

           Örnekte kullanılan GUID 8-4-4-4-12 formatında 36 karakterlik bir içerik üretir.
        */
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            (random_number >> 8) as u32,
            ((random_number >> 16) & 0xFFF) as u16,
            ((random_number >> 32) & 0xFFF) as u16,
            ((random_number >> 48) & 0xFFF) as u16,
            (random_number & 0xFFFF_FFFF_FFFF) as u64,
        )
    }
}
