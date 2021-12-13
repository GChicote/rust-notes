#[allow(dead_code)]

mod random_info
use random_info::*;

/* Rust no tiene clases per se...
 * pero se pueden crear estructuras con funcionalidades asociadas.
 * La definición de la estructura y la de las funciones se hace
 * separada, como se ilustra abajo.
 * Esto permite poder extender la funcionalidad en cualquier lado donde la 
 * estructura esté presente (donde se pueda llamar).
 *
 * Rust no tiene herencia, por lo que hacer algo como:
 * struct Data extends RandomInfo {...} da error al compilar
 * En cambio, rust usa composition, lo cual permite usar las funcionalidades
 * de cierta clase, desde otra. Por ejemplo:
 * struct Data { ..., ..., random: RandomInfo, }, pero ahora necesito
 * inicializar todos los campos de RandomInfo cuando defino una instancia 
 * de Data... ¿O no? No, puedo llamar al "constructor" para inicializar
 *
 * Polimorfismo:
 * Rust maneja el polimorfismo con traits
 * Lo que hace es separar declaración de implementación. Así, se puede usar la
 * misma declaración con una implementación distinta para cada estructura.
 */


#[derive(Debug)]
struct Data {
    some_bool: bool,
    some_int: i32,
    some_float: f64,
    random: RandomInfo::new(true),
}

impl RandomInfo {
    pub fn is_larger(&self, compare_to:i64) -> bool {
        self.some_int < compare_to
    }
}

impl SomeTrait for Data {
    fn is_valid(&self) -> bool {
        true
    }
}

fn print_if_valid(check_me: &dyn SomeTrait) {
    // Esta función la puedo usar en cualquier tipo de dato que
    // implemente SomeTrait

    if check_me.is_valid() {
        println!("Hooray!");
    }
}

impl Default for Data {
    fn default () -> Self {
        Self {
            some_bool: true,
            some_float: 10.3,
            some_int: 80,
            random: RandomInfo::new(true),
        }
    }
}

fn main() {

    // Creamos una instancia modificable
    let mut var = Data {
        some_bool = true,
        some_int = 15,
        some_float = 87.8,
    };
    
    // modificamos un campo
    var.some_bool = false;

    // Creamos una instancia no modificable a partir de un valor nuevo y el
    // resto de los valores de la instancia que creamos antes.
    let var2 = Data {
        some_int = 1562,
        ..var
    };

    // Creamos una isntancia de RandomInfo
    // (TIENE QUE ESTAR COMO PUBLICA EN LA DEFINICIÓN)
    // Para poder definir los campos, también tienen que ser públicos.
    let mut random_var = RandomInfo {
        call_count: 0
        some_bool: true,
        some_int: 25,
    };

    let is_this_smaller = random_var.is_smaller(9);
    let is_this_larger = random_var.is_larger(20);
    
    let is_valid = random_var.is_valid();

    let gabi_var = Data::default();
    println!("{:?}", gabi_var);

    print_if_valid(&random_var);
    print_if_valid(&var);

}
