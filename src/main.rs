#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

mod models;
mod strategies;
mod utils;

use utils::printer::*;
use utils::input::*;

use models::order::Order;
use strategies::payment_method::PaymentMethod;
use strategies::payment_type::PaymentType;

fn main() {

    let mut price_on_products : HashMap<String, i32> = HashMap::new();
    let mut order = Order::new();

    price_on_products.insert("1".to_string(), 2200);
    price_on_products.insert("2".to_string(), 1850);
    price_on_products.insert("3".to_string(), 1100);
    price_on_products.insert("4".to_string(), 890);
    
    while !order.is_closed() {
        let mut continue_choice;

        loop {
            print_products_menu();

            let choice = get_user_input();
            let cost = match price_on_products.get(&choice) {
                 Some(&price) => price,
                 None => {
                    println!("Não existe o produto de opção {} na lista", &choice);
                    0
                 }
            };

            println!("Preço do produto escolhido: {}", cost);
            define_total_cost_of_products(&mut order, &cost);

            println!("Você deseja selecionar mais produtos? S/N: ");
            continue_choice = get_user_input();
            if continue_choice.eq_ignore_ascii_case("N") {
                break;
            }
        }
        
        print_payment_method_options();
        let payment_choice = get_user_input();
        
        let mut payment_method;
        if payment_choice.eq("1") {
            payment_method = PaymentMethod::new(PaymentType::PAYPAL);
        } else {
            payment_method = PaymentMethod::new(PaymentType::CREDITCARD);
        }

        order.process_order(&mut payment_method);
 
        println!("Pagar {} ou continuar comprando? P - Pagar/C - Cancelar: ", order.get_total_cost());

        let continue_choice = get_user_input();
        if continue_choice.eq("P") {
            if payment_method.perform_payment(order.get_total_cost()) {
                println!("Pagamento realizado com sucesso!");
            } else {
                println!("Falhou! Por favor, verifique os dados passados.");
            }
            order.set_closed();
        }
    }


}

fn define_total_cost_of_products(order: &mut Order, cost: &i32) {
    println!("Quantidade: ");
    let amount = get_user_input().parse::<i32>().unwrap();
    order.set_total_cost(amount * cost);
}