use crate::model::prelude::*;
use crate::token::prelude::*;

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
        if let Some(class_name) = token.body.lines().find(|l| l.contains("class")) {
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

        let lines = token
            .body
            .lines()
            .filter(|l| l.contains("get") || l.contains("set"));
        for b_line in lines {
            //println!("Line : {}", b_line);
            let property = PropertyToken::parse(&b_line.to_owned()).unwrap();
            //println!("Property Info : {:?}", property);
            properties.push(property);
        }

        Ok(Class {
            name,
            properties,
            fields: vec![],
            methods: vec![],
        })
    }
}
