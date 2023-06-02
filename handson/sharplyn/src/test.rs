#[cfg(test)]
mod test {
    use crate::ast_model::{Class, Namespace};
    use crate::ast_parser::Parse;

    #[test]
    fn should_class_names_can_get_test() {
        let sharp_code = "
        using System;
        using System.Collections.Generic;

        public namespace GameBusiness {
            public class State {
            }

            public class Actor {
            }

            public class Event {
            }
        }";
        let namespace = Namespace::parse(sharp_code).unwrap();
        let classes = namespace.classes;

        assert_eq!(classes[0].name, "State");
        assert_eq!(classes[1].name, "Actor");
        assert_eq!(classes[2].name, "Event");
    }
}
