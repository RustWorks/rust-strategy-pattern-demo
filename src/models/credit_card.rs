#[allow(dead_code)]
pub struct CreditCard {
    number: String,
    date: String,
    cvv: String,
    pub amount: i32
}

impl CreditCard {
    pub fn new(number: String, date: String, cvv: String) -> CreditCard {
        CreditCard {
            amount: 30000,
            number,
            date,
            cvv
        }
    }

    pub fn get_amount(&self) -> i32 {
        self.amount
    }

    pub fn set_amount(&mut self, amount: i32) {
        self.amount = amount
    }
}