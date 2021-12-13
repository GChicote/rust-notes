/**
 * 
 * What are lifetimes?
 * Rust permite sólo un dueño para cada "pedazo" de memoria.
 * Permite multiples referencias
 * Los lifetimes solucionan el problema de los dangling pointers, punteros a
 * algo no existente.
 * Los lifetimes fuerzan a a esos pedazos de memoria a mantenerse válidos para
 * una referencia.
 */

#[allow(dead_code)]
const SOME_CONST: &str = "Soy una constante... soy inmortal";
#[allow(dead_code)]
const SOME_CONST_2: &str = "Muajajajaja!!";

#[allow(dead_code, unused_variables)]
fn main() {
    /*
    let a;
    {
        let b: String = String::from("Hooray!");
        // Cuando éste bloque termina, termina también el scope de b, y por ende
        // deja de ser válida y se elimina.
        
        a = b;
        // Con esto podemos usar el valor de b cuando se termina el bloque
        // Pero ¿Y si queremos que a sea una referencia?

        a = &b;
        // Nos da un error de compilación porque b muere cuando termina el 
        // bloque y a estaría apuntando a algo inválido.
        // ¿Y ahora...?
        // Acá entran en juego los lifetimes, pasemos a otro ejemplo para verlo
    }
    println!("{}", a);
    */
    ////////////////////////////////////////////////////////////////////////////

    /*
     let some_int_var = 10;
     let some_int_var2 = 15;
     let result_ref = get_int_ref(some_int_var, some_int_var2);
     println!("{}", result_ref);
     // Rust no tiene problema con esto porque el owner original y la referencia
     // tienen el mismo scope. Entonces sabe que result_ref nunca va a tener una
     // referencia inválida.
     */
    ////////////////////////////////////////////////////////////////////////////

    /*
     * Si quisiéramos que una variable tenga un lifetime válido durante el 
     * programa entero, podemos usar 'static, en vez de <'a, 'b, ...>
     * Ésto es lo que implementan las constantes.
     */

    /*
     * Nota: Si mi función devuelve algo que es static, necesariamente los 
     * argumentos que toma deben ser static, si es que existe la posibilidad de
     * devolver alguno de ellos.
     * O sea que algo como: some_func("Hola", SOME_CONST); no compila
     * Pero tranqui, no todo está perdido. Puedo especificar otro lifetime y 
     * listo, algo así:
     * fn some_func(param_1: &'a str, param_2: &'a str) -> &'a str;
     * y ahora no pasa nada
     *
     //let greater = some_func(SOME_CONST, SOME_CONST_2);
     //println!("{}", greater);
     */
    ////////////////////////////////////////////////////////////////////////////

    /*
     * ¿Y cómo hacemos con templates?
     * Es lo mismo de siempre, no hay diferencia realmente...
     * Hagamos un par de ejemplos, vamos a usar floats y &str porque son dos
     * tipos que ya tienen orden parcial (PartialOrd). Pero también sirve para
     * structs que haya definido uno, siempre y cuando soporten PartialOrd.
     */

    /*
     *let some_float_1 = 1.1;
     *let some_float_2 = 2.2;
     *let result_float = get_smaller(&some_float_1, &some_float_2);

     *let some_str_1 = "a";
     *let some_str_2 = "b";
     *let result_float = get_smaller(&some_str_1, &some_str_2);
     */
    ////////////////////////////////////////////////////////////////////////////

    /*
     * Veamos un ejemplo con structs
     */
    ////////////////////////////////////////////////////////////////////////////
}


/// ////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn get_int_ref<'a, 'b: 'a>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
    if param_1 > param_2 { param_1 } else { param_2 }
}

/**
 * Vamos a ver que pasa con la función de arriba.
 * Toma como parámetro dos referencias, y devuelve una de ellas.
 * Esto puede traer problemas, como ya vimos, así que hay que especificarle al 
 * compilador que está todo bien, que el lifetime de lo que devuelva se va a
 * corresponder con el del parámetro que sea.
 * Para esto, podemos usar un lifetime para cada parámetro:
 *
 * fn get_int_ref<'a, 'b: 'a>(param_1: &'a i32, param_2: &'b i32) -> &'a i32;
 *
 * ¿Qué estamos haciendo acá?
 * primero, definimos los dos lifetimes 'a y 'b, y con "'b: 'a" le decimos que
 * 'b >= 'a. Ésto es para garantizar que el lifetime del resultado no va a ser
 * mayor que el de los parámtros, y con eso evitamos el problema.
 * Ahora, para nuestro caso particular, nosotros tenemos los dos parámetros 
 * definidos en el mismo scope que la llamada a la función, lo cual nos dice que
 * los tres lifetimes (los dos parámetros y el return de la función) son iguales
 *
 * Así, podríamos simplemente poner un solo lifetime:
 * fn get_int_ref<'a>(param_1: &'a i32, param_2: &'a i32) -> &'a i32;
 */
/// ////////////////////////////////////////////////////////////////////////////

/// ////////////////////////////////////////////////////////////////////////////
/**
 * Veamos un ejemplo un poco más realista.
 * Es común tener que comparar dos &str. Digamos que queremos devolver el más
 * grande.
 */
#[allow(dead_code)]
fn get_str_ref<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
    if param_1 > param_2 { param_1 } else { param_2 }
}
/// ////////////////////////////////////////////////////////////////////////////

/// ////////////////////////////////////////////////////////////////////////////
/**
 * Ejemplo usando constantes
 */
#[allow(dead_code)]
fn some_func(param_1: &'static str, param_2: &'static str) -> &'static str {
    if param_1 > param_2 { param_1 } else { param_2 }
}
/// ////////////////////////////////////////////////////////////////////////////

/// ////////////////////////////////////////////////////////////////////////////
/**
 * Implementación de get_smaller genérica
 */
#[allow(dead_code)]
fn get_smaller<'a, T: std::cmp::PartialOrd>(param_1: &'a T, param_2: &'a T) ->&'a T {
    if param_1 < param_2 { param_1 } else { param_2 }
}
/// ////////////////////////////////////////////////////////////////////////////

/// ////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
struct GabiStruct<'a, 'b> {
    some_data_1: Vec<i32>,
    some_data_2: &'a Vec<i32>,
    some_data_3: &'b: 'a Vec<i32>,
}
/**
 * ¿Qué está pasando acá?
 * some_data_2 es una referencia , así que hay que asegurarse de que vive lo
 * suficiente. 
 * ¿Cómo?
 * Lifetimes papá!
 * También puedo tener varias referencias con lifetimes distintos
 * En este caso, además estamos especificando que 'b >= 'a.
 */
/// ////////////////////////////////////////////////////////////////////////////

