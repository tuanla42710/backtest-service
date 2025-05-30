use crate::portfolio::portfolio::*;

pub enum OrderType {
    BUY,
    SELL
}

pub struct Order {
    pub(crate) ticket : String,
    pub(crate) price : f64,
    pub(crate) volume : i64,
    pub(crate) order_type: OrderType
}

impl Order {
    pub fn make_order(&self, portfolio: &mut Portfolio){
        
        let t = match &self.order_type {
            OrderType::BUY => true,
            OrderType::SELL => false
        };
        
        // neu co phieu da co trong danh muc va lenh la lenh mua thi khong mua nua
        if portfolio.check_ticket(self.ticket.as_str()) && t {
            println!("can not execute buy because stock has been bought");
            return;
        }
        // neu co phieu da co va  lenh sell
        if portfolio.check_ticket(self.ticket.as_str()) && !t{
            portfolio.cash += self.volume as f64*self.price;
            let i = portfolio.get_index(self.ticket.as_str());
            portfolio.stocks.remove(i as usize);
        }
        
        // neu co phieu chua co va khong du tien thi khong mua duoc
        if !portfolio.check_ticket(self.ticket.as_str()) && self.volume as f64*self.price > portfolio.cash {
            println!("can not execute buy because you have not enough money");
            return
        }
        
        // neu co phieu chu co va lenh mua 
        if !portfolio.check_ticket(self.ticket.as_str()) && t {
            let stock = Stock {
                ticket : self.ticket.clone(),
                buy_price : self.price,
                volume : self.volume,
                current_price : self.price
            };
            portfolio.cash -= self.volume as f64*self.price;
            portfolio.stocks.push(stock);
        }

        
    }
}