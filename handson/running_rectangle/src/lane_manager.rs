use crate::constants::MAX_SCREEN_WIDTH;
use crate::entity::BlockSize;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub enum Column {
    Zero,
    One,
    Two,
    Three,
    Four,
}
pub struct Lane {
    pub column: Column,
    pub start: u32,
    pub end: u32,
}

impl From<u32> for Column {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            _ => Self::Zero,
        }
    }
}

impl Lane {
    pub fn new(column: Column, start: u32, end: u32) -> Self {
        Self { column, start, end }
    }
}

pub struct LaneManager {
    lanes: Vec<Lane>,
}

impl LaneManager {
    pub fn new() -> Self {
        let minus_width = BlockSize::Venti.to_size().width;
        let lanes = vec![
            Lane::new(Column::Zero, 0, 100 - minus_width),
            Lane::new(Column::One, 100, 200 - minus_width),
            Lane::new(Column::Two, 200, 300 - minus_width),
            Lane::new(Column::Three, 300, 400 - minus_width),
            Lane::new(Column::Four, 400, MAX_SCREEN_WIDTH - minus_width),
        ];
        Self { lanes }
    }

    #[allow(dead_code)]
    pub fn get_lane(&self, column: Column) -> Option<&Lane> {
        self.lanes.iter().find(|l| l.column == column)
    }

    pub fn get_lane_range(&self, column: Column) -> std::ops::Range<i32> {
        let lane = self.lanes.iter().find(|l| l.column == column).unwrap();
        lane.start as i32..lane.end as i32
    }
}
