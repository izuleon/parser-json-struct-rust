use parser::*;
mod parser;

fn main() {
    let point = Point { x: 1, y: 2 };
    let y = parser::struc_to_json(&point);
    println!("{}",y);
    let json_str="{\"x\":200,\"y\":1000}";
    let y = parser::json_to_struct(json_str);
    println!("{:?}",y);

}
// trait Parserable{
//     fn parse_struct_to_json(&self) -> String
//     where &self {
//         serde_json::to_string(&self).unwrap()
//     }
//     fn parse_json_to_struct<T>(&self) -> T {
        
//     }
// }