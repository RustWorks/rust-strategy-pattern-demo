pub trait PayStrategy {
    fn pay(&mut self, payment_amount : i32) -> bool;
    fn collect_payment_details(&mut self);
}