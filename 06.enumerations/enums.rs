/// ENUMERATIONS!!!
enum Payment {
    Cash(f32),
    CreditCard(String, f32),
    DebitCard(DebitData),
    Crypto{account_id: String, cant: f32},
}

struct DebitData {
    pub card_number: String,
    pub cant: f32,
}

fn payment_process(some_payment: Payment) {

    match some_payment {
        Payment::Cash( cant ) => {
            println!("Paying with cash: ${}", cant);
        }
        // Si no vas a usar algun campo, se pone un _ adelante del nombre o 
        // poner sólamente un _ en lugar del nombre para decirle al compilador
        // que es intencional
        Payment::CreditCard( some_string, _some_f32 ) => {
            println!("Paying with credit card: id='{}'", some_string);
        }
        Payment::DebitCard( data ) => {
            println!("Paying with debit card: id='{}', cant='{}'", data.card_number, data.cant);
        }
        Payment::Crypto{ account_id, cant } => {
            println!("Paying with crypto: account-id='{}', cant='{}'", account_id, cant);
        }
    }

}

fn main() {
    let cash_payment = Payment::Cash(100.);
    payment_process(cash_payment);

    let credit_payment = Payment::CreditCard(String::from("CC num"), 250.);
    payment_process(credit_payment);

    let debit_payment = Payment::DebitCard(DebitData{
        card_number: String::from("Debit num"),
        cant: 350.,
    });
    payment_process(debit_payment);

    let crypto_payment = Payment::Crypto{account_id: String::from("abcdef"), cant: 2000.};
    payment_process(crypto_payment);
}
