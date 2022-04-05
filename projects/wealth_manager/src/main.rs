mod stocks;
use stocks::{open_order,close_order};
use stocks::structs::order::Order;
use stocks::enums::order_types::OrderType;
use stocks::structs::stock::Stock;

//use serde_json::{json,Value};

fn main() {
    // let stock: Stock = Stock::new("MonolithAi", 45.3);
    // println!("Here is the stock name:{}",stock.name);
//     let stock: Value =json!({
//         "name":"Weiren",
//         "price": 43.17,
//         "history":[19.3,444,44]
//     });
//     println!("First price:{}", stock["history"][0] );
//     println!("Hello, world!");
//     println!("{}",stock.to_string());
// }

println!("Hello stocks!");
let mut new_order:Order = open_order(20, OrderType::Long, "bumper", 56.8, None, None);

println!("The current price is:{}", &new_order.current_value());
println!("The current profit is:{}", &new_order.current_profit());

new_order.stock.update_price(43.1);

println!("The current price is:{}", &new_order.current_value());
println!("The current profit is:{}", &new_order.current_profit());

new_order.stock.update_price(82.7);
println!("The current price is:{}", &new_order.current_value());
println!("The current profit is:{}", &new_order.current_profit());


let profit: f32 = close_order(new_order);
println!("We made {} profit", profit);

}
