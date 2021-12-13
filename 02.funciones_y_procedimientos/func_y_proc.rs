#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
#[allow(dead_code)]

fn main() {
   // FUNCIONES Y PROCEDIMIENTOS (Son prácticamente lo mismo)
   // Ambos aceptan parámetros y pueden hacer llamados a funciones o variables.
   // Las funciones devuelven un valor
   // Los procedimientos no.

    /* Vamos a llamar a la función "some_str_procedure"
     *
     * some_str_procedure("Hola wachin");       // Esto compila bien
     *
     * let string_slice_var: &str = "Hola";
     * some_str_procedure(string_slice_var);    // Esto compila bien
     *
     * let string_var = String::from("String de verdad");
     * some_str_procedure(string_var);          // Esto no compila
     * 
     * Y tiene sentido porque string no es &str.
     * Pero, pero, pero...
     * podemos hacer lo siguiente:
     * some_str_procedure(&string_var);         // Y esto sí compila!!!
     * Lócigo, porque ahí estamos pasando una &str con el valor que queremos 
     */

    /* Vamos a llamar a la función "some_string_procedure"
     *
     * Usemos la variable string_var de antes
     * some_string_procedure(string_var);       // Anda joya
     * some_string_procedure(string_var);       // Esto ya no compila
     * ¿Por qué? No puedo llamar al proc más de una vez con ese parámetro.
     * Ya veremos por qué.
     * 
     * Mientras tanto, podemos arreglarlo pasándole la referencia:
     * some_string_procedure(&string_var);
     * some_string_procedure(&string_var);
     * some_string_procedure(&string_var);
     * some_string_procedure(&string_var);
     * some_string_procedure(&string_var);
     * some_string_procedure(&string_var);
     * some_string_procedure(&string_var);
     * 
     * Y todas esas funcionan sin problemas.
     *
     * Tampoco puedo llamar al proc con string literals (aka cadenas)
     * Ej:
     * some_string_procedure("Mostrame por pantalla porfa");    // No compila
     *
     *
     */


}

fn some_function(a: f32, b: i128) -> f32 {
    println!("Soy una función");

    /* Hay varias formas de indicar el return
     * (1) 10.1
     * (2) 10.
     * (3) 10 as f32
     * (4) 10f32
     * (5) 10_f32
     * (6) return 10.1; (No estoy seguro de esta)
     * (7) las expresiones usuale( operaciones matemáticas y lógicas, etc... )
     */
    
    if a < 100 {
        //let ret_val: f32 = 10.1 * a + b;        // No compila ( son tipos != )
        let ret_val: f32 = 10.1 * a + b as f32; // Ahí va

        ret_val
    } else {
        -1.
    }

}

fn some_procedure(a: f32, b: i8) {
    println!("Hola, soy un procedimiento con a = {} y b = {}", a, b);
}

fn some_str_procedure(a: &str) {
    println!("Hola, soy un procedimiento para la &str '{}'", a);
}

fn some_string_procedure(a: String) {
    println!("Hola, soy un procedimiento para la string '{}'", a);
}
