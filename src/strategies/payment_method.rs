use super::pay_strategy::PayStrategy;
use super::pay_by_creditcard::PayByCreditCard;
use super::pay_by_paypal::PayByPaypal;
use super::payment_type::PaymentType;

pub struct PaymentMethod {
    method: Box<dyn PayStrategy>
}

impl PaymentMethod {
    pub fn new(payment_type: PaymentType) -> PaymentMethod {
        match payment_type {
            PaymentType::CREDITCARD => PaymentMethod { method : Box::new(PayByCreditCard::new()) },
            PaymentType::PAYPAL => PaymentMethod { method: Box::new(PayByPaypal::new()) }
        }
    }

    pub fn perform_collect_details(&mut self){
        self.method.collect_payment_details();
    }

    pub fn perform_payment(&mut self, amount: i32) -> bool{
        self.method.pay(amount)
    }
}