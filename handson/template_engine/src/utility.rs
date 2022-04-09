use super::prelude::*;

pub fn get_content_type(data: &str) -> ContentType {
    let is_tag_expression = check_matching_pair(data, "[@", "@]");
    let is_loop_tag =
        (check_symbol(data, "loop") && check_symbol(data, "in")) || check_symbol(data, "end loop");
    let is_if_tag = check_symbol(data, "if") || check_symbol(data, "end if");
    let is_template_variable = check_matching_pair(data, "[[", "]]");
    let return_value;

    if is_tag_expression && is_loop_tag {
        return_value = ContentType::Tag(Loop);
    } else if is_tag_expression && is_if_tag {
        return_value = ContentType::Tag(If);
    } else if is_template_variable {
        let content = get_expression_data(data);
        return_value = ContentType::TemplateVariable(content);
    } else if !is_tag_expression && !is_template_variable {
        return_value = ContentType::Literal(data.to_string());
    } else {
        return_value = ContentType::Unknown;
    }
    return_value
}

pub fn check_matching_pair(data: &str, symbol1: &str, symbol2: &str) -> bool {
    data.contains(symbol1) && data.contains(symbol2)
}

pub fn check_symbol(data: &str, symbol: &str) -> bool {
    data.contains(symbol)
}

pub fn get_expression_data(data: &str) -> ExpressionData {
    let (_, i) = get_index_for_symbol(data, '[');
    let head = data[0..i].to_string();
    let (_, k) = get_index_for_symbol(data, ']');
    let variable = data[i + 1 + 1..k].to_string();
    let tail = data[k + 1 + 1..].to_string();
    ExpressionData::new(Some(head), variable, Some(tail))
}

pub fn get_index_for_symbol(data: &str, symbol: char) -> (bool, usize) {
    let chars = data.char_indices();
    let mut does_exist = false;
    let mut index = 0;
    for (c, d) in chars {
        if d == symbol {
            does_exist = true;
            index = c;
            break;
        }
    }
    (does_exist, index)
}
