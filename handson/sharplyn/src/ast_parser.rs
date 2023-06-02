use crate::ast_model::{Class, Namespace};

pub trait Parse<T> {
    fn parse(code: &str) -> Result<T, ()>;
}

impl Parse<Namespace> for Namespace {
    fn parse(code: &str) -> Result<Self, ()> {
        let mut name = String::new();
        let mut classes = Vec::new();
        let mut lines = code.lines();
        for line in lines {
            let line = line.trim();

            if line.starts_with("using") {
                continue;
            }

            if line.contains("public namespace") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                name = parts[2].to_owned();
            } else if line.starts_with("public class") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let class_name = parts[2].to_owned();
                classes.push(Class {
                    name: class_name,
                    properties: vec![],
                    fields: vec![],
                    methods: vec![],
                });
            }
        }
        Ok(Self { name, classes })
    }
}
