

#[derive(Debug,Clone)]
pub struct Portfolio {
    pub(crate) cash : f64,
    pub(crate) stocks : Vec<Stock>
}
#[derive(Debug, Clone)]
pub struct Stock {
    pub(crate) ticket : String,
    pub(crate) buy_price : f64,
    pub(crate) volume : i64,
    pub(crate) current_price : f64
}

impl Portfolio {
    pub fn nav(&self) -> f64{
        let stock_value = {
            let mut sum = 0.0;
            for s in self.stocks.iter() {
                sum += s.volume as f64*s.current_price;
            }
            sum
        };
        stock_value + self.cash
    }
    
    pub fn check_ticket(&self, ticket: &str) -> bool{
        let e = {
            for s in self.stocks.iter() {
                if s.ticket == ticket {
                    return true;
                }
            }
            false
        };
        e
    }
    
    pub fn get_index(&self, ticket: &str) -> i32{
        
        let e = {
           for i in 0..self.stocks.len(){
               if &self.stocks[i].ticket == ticket {
                   return i as i32 ;
               }
           }
            -1
        };
        e 
    }
    
    pub fn update_price(&mut self, ticket : String, price : f64){
        if self.check_ticket(ticket.as_str()){
            let e = self.get_index(&ticket);
            self.stocks[e as usize].current_price = price;
        }
        
    }
}