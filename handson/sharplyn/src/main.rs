mod ast;
mod model;
mod test;
mod token;

fn main() {
    // let sharp_code = r#"
    //     using System;
    //     using System.Collections.Generic;
    //
    //     public namespace GameBusiness {
    //         public class State{
    //             public int Id { get; set; }
    //             private decimal Value { get; }
    //             protected String LastOwner { get; set; }
    //         }
    //
    //         public class Actor            {
    //         }
    //
    //         public class Event {}
    //     }"#;
    // let unit = parse_code(sharp_code).unwrap();
    // assert_eq!(unit.classes[0].properties.len(), 3);

    // let code = r#"
    //         public int Sum(int x,int y) {
    //
    //         }
    //     "#;
    //
    // let tokens = MethodToken::tokenize(code);
    // assert_eq!(tokens.len(), 1);
    //
    // let sum_method = MethodToken::parse(&tokens[0]).unwrap();
    // assert_eq!(sum_method.name, "Sum");
    // assert_eq!(sum_method.return_type, SharpType::Int);
    // assert_eq!(sum_method.parameters.len(), 2);
}
