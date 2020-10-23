use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub fn struc_to_json(a:&Point)->String{
    serde_json::to_string(a).unwrap()
}
pub fn json_to_struct(a:&str) -> Point{
    serde_json::from_str(a).unwrap()
}