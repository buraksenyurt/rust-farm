use crate::model::prelude::*;
use crate::token::prelude::*;
use regex::Regex;

pub struct ClassToken {
    pub body: String,
}

impl BodyTokenizer for ClassToken {
    type Token = Self;
    type Output = Class;
    fn tokenize(code: &str) -> Vec<Self> {
        let mut tokens = Vec::new();
        let mut in_class = false;
        let mut class_body = String::new();

        let lines = code.lines();
        for line in lines {
            let line = line.trim();

            if line.contains("class") {
                if in_class {
                    tokens.push(ClassToken {
                        body: class_body.trim().to_owned(),
                    });
                    class_body.clear();
                }
                in_class = true;
            }

            if in_class {
                class_body.push_str(line);
                class_body.push('\n');
            }
        }
        //println!("{}", class_body);
        tokens.push(ClassToken {
            body: class_body.trim().to_owned(),
        });

        tokens
    }

    fn parse(token: &Self::Token) -> Result<Self::Output, ()>
    where
        Self: Sized,
    {
        let mut name = String::new();
        let mut properties = Vec::new();
        if let Some(class_name) = token
            .body
            .lines()
            .find(|l| Regex::new(r"class").unwrap().is_match(l))
        {
            let class_name = class_name.trim();
            if let Some(open_brace_index) = class_name.find('{') {
                if let Some(class_keyword_index) = class_name.find("class") {
                    name = class_name[class_keyword_index + 5..open_brace_index]
                        .trim()
                        .to_owned();
                    //println!("Class name: {}", name);
                }
            }
        }

        let p_token=PropertyToken::tokenize(&token.body);
        for pt in p_token.iter(){
            let p=PropertyToken::parse(pt);
            properties.push(p.unwrap());
        }

        let mut methods = Vec::new();
        let m_token = MethodToken::tokenize(&token.body);
        for mt in m_token.iter() {
            //println!("{}", mt.name);
            let m = MethodToken::parse(mt);
            methods.push(m.unwrap());
        }

        Ok(Class {
            name,
            properties,
            fields: vec![],
            methods,
        })
    }
}
