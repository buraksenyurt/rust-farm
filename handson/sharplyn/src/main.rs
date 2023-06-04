use crate::ast::parser::parse_code;

mod ast;
mod model;
mod test;
mod token;

fn main() {
    let sharp_code = r#"
        using System;
        using System.Collections.Generic;

        public namespace GameBusiness {
            public class State{
                public int Id { get; set; }
                private decimal Value { get; }
                protected String LastOwner { get; set; }
            }

            public class Actor            {
            }

            public class Event {}
        }"#;
    let unit = parse_code(sharp_code).unwrap();
    assert_eq!(unit.classes[0].properties.len(), 3);
}
