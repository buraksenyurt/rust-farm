#[cfg(test)]
mod test {
    use crate::ast::parser::parse_code;
    use crate::model::prelude::SharpType;
    use crate::token::prelude::*;

    #[test]
    fn should_class_names_can_get_test() {
        let sharp_code = r#"
        using System;
        using System.Collections.Generic;

        public namespace GameBusiness {
            public class State {
            }

            public class Actor            {
            }

            public class Event {}
        }"#;
        let unit = parse_code(sharp_code).unwrap();

        assert_eq!(unit.classes[0].name, "State");
        assert_eq!(unit.classes[1].name, "Actor");
        assert_eq!(unit.classes[2].name, "Event");
    }

    #[test]
    fn should_using_names_can_get_test() {
        let sharp_code = r#"
        using System;
        using System.Collections.Generic;

        public namespace GameBusiness {
            public class State{
            }

            public class Actor            {
            }

            public class Event {}
        }"#;
        let unit = parse_code(sharp_code).unwrap();
        assert_eq!(unit.usings[0].name, "System");
        assert_eq!(unit.usings[1].name, "System.Collections.Generic");
    }

    #[test]
    fn should_namespace_can_get_test() {
        let sharp_code = r#"
        using System;
        using System.Collections.Generic;

        public namespace GameBusiness {
            public class State{
            }

            public class Actor            {
            }

            public class Event {}
        }"#;
        let unit = parse_code(sharp_code).unwrap();
        assert_eq!(unit.namespaces[0].name, "GameBusiness");
    }

    #[test]
    fn should_class_properties_can_get_test() {
        let sharp_code = r#"
            public class State{
                public int Id { get; set; }
                private decimal Value { get; }
                protected string LastOwner { get; set; }
            }
        "#;
        let property_tokens = PropertyToken::tokenize(sharp_code);
        assert_eq!(property_tokens.len(), 3);
        let id = PropertyToken::parse(&property_tokens[0]).unwrap();
        assert_eq!(id.name, "Id");
        assert_eq!(id.p_type, SharpType::Int)
    }

    #[test]
    fn should_properties_in_namespaces_can_get_test() {
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

    #[test]
    fn should_method_blocks_can_parse_test() {
        let code = r#"
            public int Sum(int x,int y) {

            }
        "#;

        let tokens = MethodToken::tokenize(code);

        assert_eq!(tokens.len(), 1);

        let sum_method = MethodToken::parse(&tokens[0]).unwrap();
        assert_eq!(sum_method.name, "Sum");
        assert_eq!(sum_method.return_type, SharpType::Int);
        assert_eq!(sum_method.parameters.len(), 2);
    }

    #[test]
    fn should_methods_can_parse_from_full_code_test() {
        let sharp_code = r#"
        using System;
        using System.Collections.Generic;

        public namespace GameBusiness {
            public class State{
                public int Id { get; set; }
                private decimal Value { get; }
                protected String LastOwner { get; set; }

                public int Sum(int x,int y)
                {

                }

                public string ConvertTo(double value)
                {

                }
            }

            public class Actor            {
            }

            public class Event {}
        }"#;
        let unit = parse_code(sharp_code).unwrap();
        assert_eq!(unit.classes[0].methods.len(), 2);
        assert_eq!(unit.classes[0].methods[0].name, "Sum");
        assert_eq!(unit.classes[0].methods[1].name, "ConvertTo");
        assert_eq!(unit.classes[0].methods[0].parameters[0].name,"x");
        assert_eq!(unit.classes[0].methods[0].parameters[1].name,"y");
        assert_eq!(unit.classes[0].methods[1].parameters[0].name,"value");
    }
}
