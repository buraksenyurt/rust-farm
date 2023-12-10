fn calc_mines_count(row_idx: usize, col_idx: usize, minefield: &[&str]) -> usize {
    let mut total_mines = 0;
    let rows_count = minefield.len() as isize;
    let cols_count = minefield[0].len() as isize;

    for i in -1..=1 {
        for j in -1..=1 {
            let (new_row, new_col) = (row_idx as isize + i, col_idx as isize + j);

            if new_row >= 0
                && new_row < rows_count
                && new_col >= 0
                && new_col < cols_count
                && !(i == 0 && j == 0)
            {
                if let Some('*') = minefield[new_row as usize].chars().nth(new_col as usize) {
                    total_mines += 1;
                }
            }
        }
    }

    total_mines
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    //println!("Incoming request {:?}", minefield);
    for (row_idx, row) in minefield.iter().enumerate() {
        let mut annotated_row = String::new();
        let row_chars: Vec<char> = row.chars().collect();

        for (col_index, &cell) in row_chars.iter().enumerate() {
            if cell == '*' {
                annotated_row.push('*');
            } else {
                let mine_count = calc_mines_count(row_idx, col_index, minefield);
                if mine_count > 0 {
                    annotated_row.push_str(&mine_count.to_string());
                } else {
                    annotated_row.push(' ');
                }
            }
        }
        result.push(annotated_row);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    fn remove_annotations(board: &[&str]) -> Vec<String> {
        board.iter().map(|r| remove_annotations_in_row(r)).collect()
    }

    fn remove_annotations_in_row(row: &str) -> String {
        row.chars()
            .map(|ch| match ch {
                '*' => '*',
                _ => ' ',
            })
            .collect()
    }

    fn run_test(test_case: &[&str]) {
        let cleaned = remove_annotations(test_case);
        let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
        let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, annotate(&cleaned_strs));
    }

    #[test]

    fn no_rows() {
        #[rustfmt::skip]
        run_test(&[
        ]);
    }

    #[test]
    fn no_columns() {
        #[rustfmt::skip]
        run_test(&[
            "",
        ]);
    }

    #[test]
    fn no_mines() {
        #[rustfmt::skip]
        run_test(&[
            "   ",
            "   ",
            "   ",
        ]);
    }

    #[test]
    fn board_with_only_mines() {
        #[rustfmt::skip]
        run_test(&[
            "***",
            "***",
            "***",
        ]);
    }

    #[test]
    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
        run_test(&[
            "111",
            "1*1",
            "111",
        ]);
    }

    #[test]
    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
        run_test(&[
            "***",
            "*8*",
            "***",
        ]);
    }

    #[test]
    fn horizontal_line() {
        #[rustfmt::skip]
        run_test(&[
            "1*2*1",
        ]);
    }

    #[test]
    fn horizontal_line_mines_at_edges() {
        #[rustfmt::skip]
        run_test(&[
            "*1 1*",
        ]);
    }

    #[test]
    fn vertical_line() {
        #[rustfmt::skip]
        run_test(&[
            "1",
            "*",
            "2",
            "*",
            "1",
        ]);
    }

    #[test]
    fn vertical_line_mines_at_edges() {
        #[rustfmt::skip]
        run_test(&[
            "*",
            "1",
            " ",
            "1",
            "*",
        ]);
    }

    #[test]
    fn cross() {
        #[rustfmt::skip]
        run_test(&[
            " 2*2 ",
            "25*52",
            "*****",
            "25*52",
            " 2*2 ",
        ]);
    }

    #[test]
    fn large_board() {
        #[rustfmt::skip]
        run_test(&[
            "1*22*1",
            "12*322",
            " 123*2",
            "112*4*",
            "1*22*2",
            "111111",
        ]);
    }
}
