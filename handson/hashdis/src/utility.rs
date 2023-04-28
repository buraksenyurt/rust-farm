use bytes::BytesMut;
use log::info;

pub struct Utility;

impl Utility {
    pub fn convert_to_vec(buffer: &mut BytesMut) -> Vec<String> {
        let incoming = buffer.to_vec();
        let mut keywords: Vec<String> = vec![];
        let mut keyword = "".to_string();
        info!("Veri büyüklüğü {} byte", buffer.len());

        for i in 0..incoming.len() {
            match incoming[i] {
                b' ' => {
                    keywords.push(keyword);
                    keyword = "".to_string();
                }
                c => {
                    keyword.push(c as char);
                    let copy_keyword = keyword.clone();
                    if i == incoming.len() - 1 {
                        keywords.push(copy_keyword);
                    }
                }
            }
        }
        keywords
    }
}
