use std::collections::HashMap;

use super::pay_strategy::PayStrategy;
use crate::utils::input::*;

lazy_static! {
    static ref DATABASE: HashMap<String, &'static str> = {
        let mut map = HashMap::new();
        map.insert(String::from("amanda1995"), "amanda@yahoo.com");
        map.insert(String::from("tomas1990"), "tomas@uol.com.br");
        map
    };
}

pub struct PayByPaypal {
    email: String,
    password: String,
    signed_in: bool
}

impl PayStrategy for PayByPaypal {
    fn pay(&mut self, payment_amount: i32) -> bool {
        if self.signed_in {
            println!("Pagando {} usando PayPal", payment_amount);
            true
        } else {
            false
        }
        
    }

    fn collect_payment_details(&mut self) {
        println!("Coletando dados para pagamento com Paypal");
        while !self.signed_in {
            println!("Insira o email do usuário: ");
            self.email = get_user_input();
            println!("Insira a senha: ");
            self.password = get_user_input();

            if self.verify() {
                println!("Verificação de dados bem sucedido!");
            } else {
                println!("Password ou email invalidos!");
            }

        }
    }
}

impl PayByPaypal {
    pub fn new() -> PayByPaypal {
        PayByPaypal {
            email: String::from(""),
            password: String::from(""),
            signed_in: false
        }
    }

    fn verify(&mut self) -> bool {
        let user_password = DATABASE.get(&self.password).unwrap();
        self.signed_in = self.email.eq(user_password);
        self.signed_in
    }
}