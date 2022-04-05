use super::utils::consturctor_shout;

pub struct Stock {
    pub name: String,
    pub open_price: f32,
    pub stop_loss: f32,
    pub take_profit: f32,
    pub current_price:f32
}

impl Stock {
    pub fn new(stock_name: &str, price: f32)-> Stock {
        consturctor_shout(stock_name);
        return Stock{
            name: String::from(stock_name),
            open_price: price,
            stop_loss: 0.0,
            take_profit: 0.0, 
            current_price: price 
        }
    }

    pub fn with_stop_loss(mut self, value:f32)-> Stock{
        self.stop_loss = value;
        return self
    }

    pub fn with_take_profit(mut self, value:f32)-> Stock{
        self.take_profit=value;
        return self
    }

    pub fn update_price(&mut self, value:f32){
        self.current_price=value;
    }
}