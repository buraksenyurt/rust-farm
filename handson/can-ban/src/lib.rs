use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WorkItem {
    id: u32,
    title: String,
    duration: u32,
    duration_type: DurationType,
    size:Size
}

#[wasm_bindgen]
pub enum DurationType {
    Hour,
    Day,
    Week,
    Month,
}

#[wasm_bindgen]
pub enum Size {
    Small,
    Medium,
    Large,
    Epic,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }