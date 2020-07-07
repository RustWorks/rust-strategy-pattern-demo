use crate::strategies::payment_method::PaymentMethod;

pub struct Order {
    is_closed: bool,
    total_cost: i32
}

impl Order {

    pub fn new() -> Order {
        Order {
            is_closed: false,
            total_cost: 0
        }
    }

    pub fn process_order(&self, payment_method: &mut PaymentMethod) {
        payment_method.perform_collect_details();
    }

    pub fn get_total_cost(&self) -> i32 {
        self.total_cost
    }

    pub fn set_total_cost(&mut self, value: i32) {
        self.total_cost = value
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn set_closed(&mut self) {
        self.is_closed = true;
    }

}