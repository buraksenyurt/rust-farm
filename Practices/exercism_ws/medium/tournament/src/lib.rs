use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Team<'a> {
    pub name: &'a str,
    pub win: u16,
    pub lose: u16,
    pub draw: u16,
}
impl<'a> Team<'a> {
    pub fn new(name: &'a str, win: u16, lose: u16, draw: u16) -> Self {
        Self {
            name,
            win,
            lose,
            draw,
        }
    }
}

impl<'a> Display for Team<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{:name_region$} | {:>point_region$} | {:>point_region$} | {:>point_region$} | {:>point_region$} | {:>point_region$}",
            self.name,
            self.win + self.draw + self.lose,
            self.win,
            self.draw,
            self.lose,
            3 * self.win + self.draw,
            name_region = 30,
            point_region = 2
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table = format!(
        "{:w_1$} | {:>w_2$} | {:>w_2$} | {:>w_2$} | {:>w_2$} | {:>w_2$}",
        "Team",
        "MP",
        "W",
        "D",
        "L",
        "P",
        w_1 = 30,
        w_2 = 2
    );

    let mut map: HashMap<String, Team> = HashMap::new();
    for line in match_results.lines() {
        let mut peeker = line.split(';').enumerate().peekable();
        let home = peeker.peek().unwrap().1;
        peeker.next();
        let visitor = peeker.peek().unwrap().1;
        peeker.next();
        let match_result = peeker.peek().unwrap().1;

        if !map.contains_key(home) {
            map.insert(home.to_string(), Team::new(home, 0, 0, 0));
        }
        if !map.contains_key(visitor) {
            map.insert(visitor.to_string(), Team::new(visitor, 0, 0, 0));
        }

        match match_result {
            "win" => {
                map.get_mut(home).unwrap().win += 1;
                map.get_mut(visitor).unwrap().lose += 1;
            }
            "loss" => {
                map.get_mut(home).unwrap().lose += 1;
                map.get_mut(visitor).unwrap().win += 1;
            }
            "draw" => {
                map.get_mut(home).unwrap().draw += 1;
                map.get_mut(visitor).unwrap().draw += 1;
            }
            _ => (),
        }
    }

    let mut v: Vec<&Team> = map.values().collect::<Vec<_>>();
    v.sort_by(|home_stats, visitor_stats| {
        let home_points = home_stats.win * 3 + home_stats.draw;
        let visitor_points = visitor_stats.win * 3 + visitor_stats.draw;
        if home_points == visitor_points {
            home_stats.name.cmp(&visitor_stats.name)
        } else {
            visitor_points.cmp(&home_points)
        }
    });

    for stats in v {
        table.push_str(stats.to_string().as_str());
    }

    return table;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_the_header_if_no_input() {
        let input: &[&str] = &[];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = ["Team                           | MP |  W |  D |  L |  P"].join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn a_win_is_three_points_a_loss_is_zero_points() {
        let input: &[&str] = &["Allegoric Alaskans;Blithering Badgers;win"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3",
            "Blithering Badgers             |  1 |  0 |  0 |  1 |  0",
        ]
        .join("\n");

        assert_eq!(output, expected);
    }

    #[test]
    fn a_win_can_also_be_expressed_as_a_loss() {
        let input: &[&str] = &["Blithering Badgers;Allegoric Alaskans;loss"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3",
            "Blithering Badgers             |  1 |  0 |  0 |  1 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn a_different_team_can_win() {
        let input: &[&str] = &["Blithering Badgers;Allegoric Alaskans;win"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Blithering Badgers             |  1 |  1 |  0 |  0 |  3",
            "Allegoric Alaskans             |  1 |  0 |  0 |  1 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn a_draw_is_one_point_each() {
        let input: &[&str] = &["Allegoric Alaskans;Blithering Badgers;draw"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  1 |  0 |  1 |  0 |  1",
            "Blithering Badgers             |  1 |  0 |  1 |  0 |  1",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn there_can_be_more_than_one_match() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;win",
            "Allegoric Alaskans;Blithering Badgers;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6",
            "Blithering Badgers             |  2 |  0 |  0 |  2 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn there_can_be_more_than_one_winner() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;loss",
            "Allegoric Alaskans;Blithering Badgers;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  2 |  1 |  0 |  1 |  3",
            "Blithering Badgers             |  2 |  1 |  0 |  1 |  3",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn there_can_be_more_than_two_teams() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;win",
            "Blithering Badgers;Courageous Californians;win",
            "Courageous Californians;Allegoric Alaskans;loss",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6",
            "Blithering Badgers             |  2 |  1 |  0 |  1 |  3",
            "Courageous Californians        |  2 |  0 |  0 |  2 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn typical_input() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;win",
            "Devastating Donkeys;Courageous Californians;draw",
            "Devastating Donkeys;Allegoric Alaskans;win",
            "Courageous Californians;Blithering Badgers;loss",
            "Blithering Badgers;Devastating Donkeys;loss",
            "Allegoric Alaskans;Courageous Californians;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7",
            "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6",
            "Blithering Badgers             |  3 |  1 |  0 |  2 |  3",
            "Courageous Californians        |  3 |  0 |  1 |  2 |  1",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn incomplete_competition_not_all_pairs_have_played() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;loss",
            "Devastating Donkeys;Allegoric Alaskans;loss",
            "Courageous Californians;Blithering Badgers;draw",
            "Allegoric Alaskans;Courageous Californians;win",
        ];

        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6",
            "Blithering Badgers             |  2 |  1 |  1 |  0 |  4",
            "Courageous Californians        |  2 |  0 |  1 |  1 |  1",
            "Devastating Donkeys            |  1 |  0 |  0 |  1 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn ties_broken_alphabetically() {
        let input: &[&str] = &[
            "Courageous Californians;Devastating Donkeys;win",
            "Allegoric Alaskans;Blithering Badgers;win",
            "Devastating Donkeys;Allegoric Alaskans;loss",
            "Courageous Californians;Blithering Badgers;win",
            "Blithering Badgers;Devastating Donkeys;draw",
            "Allegoric Alaskans;Courageous Californians;draw",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  3 |  2 |  1 |  0 |  7",
            "Courageous Californians        |  3 |  2 |  1 |  0 |  7",
            "Blithering Badgers             |  3 |  0 |  1 |  2 |  1",
            "Devastating Donkeys            |  3 |  0 |  1 |  2 |  1",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn ensure_points_sorted_numerically() {
        let input: &[&str] = &[
            "Devastating Donkeys;Blithering Badgers;win",
            "Devastating Donkeys;Blithering Badgers;win",
            "Devastating Donkeys;Blithering Badgers;win",
            "Devastating Donkeys;Blithering Badgers;win",
            "Blithering Badgers;Devastating Donkeys;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Devastating Donkeys            |  5 |  4 |  0 |  1 | 12",
            "Blithering Badgers             |  5 |  1 |  0 |  4 |  3",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
}
