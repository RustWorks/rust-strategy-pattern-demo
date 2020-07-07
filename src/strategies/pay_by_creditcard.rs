use super::pay_strategy::PayStrategy;
use crate::utils::input::*;
use crate::models::credit_card::CreditCard;

pub struct PayByCreditCard {
    card: Option<CreditCard>
}

impl PayStrategy for PayByCreditCard {

    fn pay(&mut self, payment_amount: i32) -> bool {
        if let Some(card) = &mut self.card {
            println!("Pagando {} usando Cartão de Crédito", payment_amount);
            card.set_amount(card.get_amount() - payment_amount);
            true
        } else {
            println!("Cartão de crédito não informado");
            false
        }
    }

    fn collect_payment_details(&mut self) {
        println!("Coletando dados para pagamento com cartão de crédito.");
        println!("Insira o número do cartão: ");
        let number = get_user_input();
        println!("Insira a data de expiração 'mm/aa': ");
        let date = get_user_input();
        println!("Insira o código CVV: ");
        let cvv = get_user_input();
        self.card = Some(CreditCard::new(number, date, cvv));
    }
}

impl PayByCreditCard {
    pub fn new() -> PayByCreditCard {
        PayByCreditCard {
            card : None
        }
    }
}