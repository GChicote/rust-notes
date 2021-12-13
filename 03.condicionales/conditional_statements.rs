#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
#[allow(dead_code)]

fn main() {

    // Es casi todo igual a como ya sabemos.
    // Lo interesante es el match ( el switch de rust )

    let n: u8 = 159;

    match n {
        0 | 1  => println!("n in { 0, 1 }"),
        2..=50 => println!("2 <= n <= 50"),
        _      => println!("n > 50"),   
    }
    
    // También puedo usarlo para asignar cosas o como un return

    let m: i8 = 9;
    let nueva_var: String = match m {
       m % 2 == 0 => "m es par",
       m % 2 != 0 => "m es impar",
       _          => "m no es par ni impar... wait, what?",
    }

}
