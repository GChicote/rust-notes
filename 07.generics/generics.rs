//use std::fmt::*;
/*
 * // Esto es una estructura explícita para i32
 * struct Punto {
 *     x: i32,
 *     y: i32,
 * }
 * 
 * // Esto es una estructura explícita para f64
 * struct Punto {
 *     x: f64,
 *     y: f64,
 * }
 * 
 * El compilador no me deja tener la misma estructura para diferentes tipos
 * Acá entran los generics (los templates de C++)
 * Los generics, o templates, son un tipo abstracto que me permiten usar una
 * misma estructura o función para diferentes tipos para más de un tipo.
 * Y el tipo que se va a usar, se determina en compiletime, no runtime... golazo
 * Para definirlos, como en C++, se usa el "placeholder" <T>. Puedo usar tantos
 * tipos como quiera, y mezclar dentro de una misma struct
 * Se pueden usar para structs, enums, funciones, y potencalmente para cosas
 * que todavía no sé que existen
 *
 * struct Estr<T, U> {
 *     a: T,
 *     b: U,
 *     c: String,
 * }
 *
 */

/* Para enums:
 *
 * enum SomeEnum<T> {
 *    OptionA(T),
 *    OptionB(T),
 *    OptionC,
 * }
 *
 */

// EJEMPLO DE struct
struct Punto<T> {
    x: T,
    y: T,
}

// EJEMPLO DE enum
#[derive(Debug)]
enum SomeEnum<T> {
    OptionA(T),
    OptionB(T),
    OptionC(T),
}

// EJEMPLO DE función
// Le tengo que decir que voy a devolver un tipo T, que tomo dos tipos T, que
// con los tipos T voy a querer hacer una suma (Por eso el "T: std::ops::Add"),
// y como el tipo de retorno podría no resultar T, agrego el  "<Output=T>"
// como además voy a querer restar, agrego "std::ops::Sub<Output=T>"
// También, voy a quere usar los debug prints, so... agrego "std::fmt::Debug"
#[allow(dead_code)]
fn gabi_fn<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug>(input_a: T, input_b: T) -> T {
    println!("input_a has {:?}", input_a);
    input_a + input_b
}

/* Pero Gabi, ésto es asqueroso, mirá esa cochinada, no se entiende nada
 * Tranqui, hay una forma más legible de escribir todo eso, y ya que estamos, 
 * agregamos otro tipo U (podría ser tipo E, J, etc):
 */
fn gabi_fn_linda<T, Pepe>(input_a: T, input_b: T, input_c: Pepe) -> T
where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug,
      Pepe: std::fmt::Debug {
    println!("input_a has {:?}", input_a);
    println!("input_c has {:?}", input_c);
    input_a + input_b
}

/* Bueno, puede que te sientas estafado, yo también me esperaba una sintaxis 
 * mucho más compacta y agradable a la vista, pero eso es efectivamente más 
 * fácil de leer
 */


/* El objetivo de esto no es programar para tipos específicos, sino para
 * habilidades o capacidades de tipos. Estas habilidades o capacidades están
 * definidas por los traits
 */

/// TRAITS
/* Vamos a crear nuestro propio trait
 */
trait CustomTrait {
    fn asd_asd(&self, a: &str, b:&str) -> String;
}

// Ahora definamos una función genérica para nuestro trait
fn hace_algo<T>(var: &T) -> String
where T: CustomTrait + std::fmt::Debug {
    // Custom Logic
    println!("{:?}", var);
    var.asd_asd("prim", "fin")
}

// También lo podría hacer de la siguiente manera, ¿No?
fn hace_algo2(var: &dyn CustomTrait) -> String {
    // println!("{:?}", var);   // Se rompe en esta línea 
    var.asd_asd("prim", "fin")
}
// Bueno... no. Esto no compila porque no tengo definido el debug print y no sé
// si hay una manera elegante de definirlo.

// Definamos una struct y su impl para este trait
#[derive(Debug)] // Necesario para que compile porque la función hace_algo
                 // definida para el CustomTrait implementa el debug print {:?}
struct GabiStruct {
    algo: i32,
}

impl CustomTrait for GabiStruct {
    fn asd_asd(&self, a: &str, b:&str) -> String {
        self.algo.to_string() + " - " + a + " - " + b
    }
}

// Pero también queremos que CustomTrait esté implementado para i32
impl CustomTrait for i32 {
    fn asd_asd(&self, a: &str, b:&str) -> String {
        "i32".to_string() + " - " + a + " - " + b
    }
}

// Veamos bloques de implementación para estructuras genéricas
struct StefStruct<T, U> {
    stef_t: T,
    stef_u: U,
}

impl<T, U> StefStruct<T, U>
where T: std::fmt::Debug,
      U: std::fmt::Debug {
    fn logea_algo(&self) {
        println!("{:?} {:?}", self.stef_t, self.stef_u);
        // Para que esto compile ¡HAY QUE DEFINIR EL DEBUG PRINT!
        // Pero ésta vez, se define sobre la impl, no sobre el struct
        // Rust things...
    }
}


#[allow(non_snake_case)]
fn main() {
    // Structs:
    let a = Punto { x: 100, y: -1 };
    println!("(x,y) = ({},{})", a.x, a.y);

    let b = Punto { x: 10.1, y: -2.3 };
    println!("(x,y) = ({},{})", b.x, b.y);

    // Enums:
    let _dataA = SomeEnum::OptionA(34.2);
    let _dataB = SomeEnum::OptionB(String::from("Gabi"));
    let _dataC = SomeEnum::OptionC(vec![1,2,3,4,5]);
    println!("dataA has {:?}", _dataA);
    println!("dataB has {:?}", _dataB);
    println!("dataC has {:?}", _dataC);

    // Funciones:
    let x = gabi_fn_linda(3.14, 2.71, "Gabi");
    println!("x has {:?}", x);

    // Traits:
    let test = GabiStruct { algo: 1000 };
    let res = hace_algo(&test);

    let testi32 = 18;
    let resi32 = hace_algo(&testi32);
    /* Esto de arriba no va a compilar porque CustomTrait no está
     * implementado para integers.
     * Pero nada nos impide extender otros tipos!
     * Y esto se puede hacer para CUALQUIER tipo. ¡¡Sí, incluso los primitivos!!
     * Por eso Rust separa la implementación de los traits.
     */

    let test2 = StefStruct {
        stef_t: 5.6,
        stef_u: vec![1,2,3,4,5],
    };

    test2.logea_algo();
    
}
