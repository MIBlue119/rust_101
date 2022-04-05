mod stocks;
use stocks::structs::stock::Stock;

//use serde_json::{json,Value};

fn main() {
    let stock: Stock = Stock::new("MonolithAi", 45.3);
    println!("Here is the stock name:{}",stock.name);
//     let stock: Value =json!({
//         "name":"Weiren",
//         "price": 43.17,
//         "history":[19.3,444,44]
//     });
//     println!("First price:{}", stock["history"][0] );
//     println!("Hello, world!");
//     println!("{}",stock.to_string());
// }

}
