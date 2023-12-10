#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_in_the_middle<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    for i in 0..=(second_list.len() - first_list.len()) {
        if first_list
            .iter()
            .zip(&second_list[i..])
            .all(|(a, b)| a == b)
        {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.is_empty(), second_list.is_empty()) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        _ => {
            if first_list.len() == second_list.len() {
                if first_list.eq(second_list) {
                    return Comparison::Equal;
                }
            } else if first_list.len() < second_list.len() {
                if second_list.starts_with(first_list)
                    || second_list.ends_with(first_list)
                    || is_in_the_middle(first_list, second_list)
                {
                    return Comparison::Sublist;
                }
            } else if first_list.len() > second_list.len()
                && (first_list.starts_with(second_list)
                    || first_list.ends_with(second_list)
                    || is_in_the_middle(second_list, first_list))
            {
                return Comparison::Superlist;
            }
            Comparison::Unequal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lists() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }

    #[test]
    fn empty_list_within_non_empty_list() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn non_empty_list_contains_empty_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn list_equals_itself() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }

    #[test]
    fn different_lists() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[2, 3, 4];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn false_start() {
        let list_one: &[i32] = &[1, 2, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 1, 2, 5, 6];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn consecutive() {
        let list_one: &[i32] = &[1, 1, 2];
        let list_two: &[i32] = &[0, 1, 1, 1, 2, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn sublist_at_start() {
        let list_one: &[i32] = &[0, 1, 2];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn sublist_in_middle() {
        let list_one: &[i32] = &[2, 3, 4];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn sublist_at_end() {
        let list_one: &[i32] = &[3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn at_start_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn in_middle_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn at_end_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn first_list_missing_element_from_second_list() {
        let list_one: &[i32] = &[1, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn second_list_missing_element_from_first_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn first_list_missing_additional_digits_from_second_list() {
        let list_one: &[i32] = &[1, 2];
        let list_two: &[i32] = &[1, 22];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn order_matters_to_a_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[3, 2, 1];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn same_digits_but_different_numbers() {
        let list_one: &[i32] = &[1, 0, 1];
        let list_two: &[i32] = &[10, 1];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
}
