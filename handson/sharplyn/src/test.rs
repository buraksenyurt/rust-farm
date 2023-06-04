#[cfg(test)]
mod test {
    use crate::ast::parser::parse_code;

    #[test]
    fn should_class_names_can_get_test() {
        let sharp_code = "
        using System;
        using System.Collections.Generic;

        public namespace GameBusiness {
            public class State{
            }

            public class Actor            {
            }

            public class Event {}
        }";
        let unit = parse_code(sharp_code).unwrap();

        assert_eq!(unit.classes[0].name, "State");
        assert_eq!(unit.classes[1].name, "Actor");
        assert_eq!(unit.classes[2].name, "Event");
    }

    #[test]
    fn should_using_names_can_get_test() {
        let sharp_code = "
        using System;
        using System.Collections.Generic;

        public namespace GameBusiness {
            public class State{
            }

            public class Actor            {
            }

            public class Event {}
        }";
        let unit = parse_code(sharp_code).unwrap();
        assert_eq!(unit.usings[0].name, "System");
        assert_eq!(unit.usings[1].name, "System.Collections.Generic");
    }

    #[test]
    fn should_namespace_can_get_test() {
        let sharp_code = "
        using System;
        using System.Collections.Generic;

        public namespace GameBusiness {
            public class State{
            }

            public class Actor            {
            }

            public class Event {}
        }";
        let unit = parse_code(sharp_code).unwrap();
        assert_eq!(unit.namespace.name, "GameBusiness");
    }
}
