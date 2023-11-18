use crate::constants::*;
use crate::issue::Issue;
use crate::json::{Deserializer, Serializer};
use crate::request::{Request, RequestMethod};
use crate::response::{HttpResponse, Response};
use crate::utility::Utility;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::MutexGuard;

pub trait Handler {
    fn handle(&mut self, stream: TcpStream, issues: MutexGuard<Vec<Issue>>);
}

pub struct ReadRequestHandler<'a> {
    pub host_address: &'a str,
}

impl<'a> Handler for ReadRequestHandler<'a> {
    fn handle(&mut self, mut stream: TcpStream, issues: MutexGuard<Vec<Issue>>) {
        let buffer_reader = BufReader::new(&mut stream);
        let request_line = buffer_reader.lines().next().unwrap().unwrap();
        println!("{}", request_line);
        let request = Request::from_str(request_line.as_str());
        match request {
            Ok(req) => match req.method {
                RequestMethod::DELETE => {
                    println!("Silme talebi geldi. {}", request_line);
                    Utility::send_response(&mut stream, String::default(), HttpResponse::Ok);
                }
                RequestMethod::GET => {
                    let issue_part: Vec<&str> = req.route.split('/').collect();
                    if issue_part.len() == 2 {
                        let json_output = Utility::vec_to_json(issues.to_vec());
                        Utility::send_response(&mut stream, json_output, HttpResponse::Ok);
                        return;
                    }
                    if let Some(issue_number_str) = issue_part.get(2) {
                        if let Ok(id) = issue_number_str.parse::<i32>() {
                            println!("Talep edilen issue ID: {}", id);
                            let issue = issues.iter().find(|i| i.id == id);
                            match issue {
                                Some(record) => {
                                    println!("{} bulundu", record.id);
                                    let response =
                                        Response::new(HttpResponse::Ok, record.to_json());
                                    stream.write_all(response.to_string().as_bytes()).unwrap();
                                }
                                None => Utility::send_response(
                                    &mut stream,
                                    String::default(),
                                    HttpResponse::NotFound,
                                ),
                            };
                        } else {
                            Utility::send_response(
                                &mut stream,
                                String::default(),
                                HttpResponse::BadRequest,
                            );
                        }
                    }
                }
                RequestMethod::POST => {}
                RequestMethod::PUT => {}
            },
            Err(_) => {
                println!("Geçerli bir talep değil!");
                Utility::send_response(&mut stream, String::default(), HttpResponse::NotFound);
            }
        }
    }
}

pub struct WriteResponseHandler<'a> {
    pub host_address: &'a str,
}

impl<'a> Handler for WriteResponseHandler<'a> {
    fn handle(&mut self, mut stream: TcpStream, mut issues: MutexGuard<Vec<Issue>>) {
        let mut buffer = [0; 1024];
        let data_size = stream.read(&mut buffer).unwrap();
        println!("{} byte içerik geldi", data_size);
        let input = String::from_utf8_lossy(&buffer[..]).to_string();
        let line_count = input.lines().count();
        println!("{}\n", input);
        let request = Request::from_str(input.lines().next().unwrap());
        match request {
            Ok(req) => {
                match req.method {
                    RequestMethod::POST => {
                        let mut json_body = String::new();
                        input
                            .lines()
                            .skip(POST_SKIP_COUNT)
                            .take(line_count - POST_SKIP_COUNT)
                            .for_each(|line| json_body.push_str(line.trim()));
                        //println!("{}", json_body);
                        let issue = <Issue as Deserializer>::from(json_body.as_str()).unwrap();
                        issues.push(issue);
                        //println!("Deserialized:\n{:?}", issue);
                        Utility::send_response(
                            &mut stream,
                            String::default(),
                            HttpResponse::Created,
                        );
                    }
                    _ => {
                        Utility::send_response(
                            &mut stream,
                            String::default(),
                            HttpResponse::BadRequest,
                        );
                    }
                }
            }
            Err(_) => {
                Utility::send_response(&mut stream, String::default(), HttpResponse::BadRequest);
            }
        }
    }
}
