pub mod structs;
pub mod enums;
use structs::stock::Stock;
use structs::order::Order;
use enums::order_types::OrderType;



pub fn close_order(order:Order)->f32{
    println!("order for {} is being close", &order.stock.name);
    return order.current_profit()
}

pub fn open_order(number:i32, order_type:OrderType,stock_name:&str, open_price:f32, stop_loss:Option<f32>,take_profit:Option<f32>)-> Order {
    println!("order for {} is being made", &stock_name);
    let mut stock: Stock = Stock::new(stock_name, open_price);
    match stop_loss{
        Some(value)=>stock=stock.with_stop_loss(value),
        None=>{}
    }

    match take_profit{
        Some(value)=> stock=stock.with_take_profit(value),
        None=>{}
    }
    return Order::new(stock, number,order_type)
}