#[allow(unused_variables)]

static mut MY_STATIC_VARIABLE: i32 = 10;

fn main() {

    ////////////////////////////// CASTING /////////////////////////////////////
    /*
     * Rust no tiene casting implícito, hay que hacerlo a manopla
     * Con la keyword "as", podemos hacerlo sin complicaciones.
     * Por ejemplo, podemos pasar some_f64 a i32 haciendo:
     *      some_f64 as i32;
     * Pero esto podría no ser conveniente, estamos perdiendo información
     * Si imprimimos eso, obtenemos 30, ¡PERDIMOS LAS 2 DÉCIMAS! Tiene sentido,
     * la única forma de pasar un float a entero, es redondeando, no?
     * Por suerte podemos hacer casting en cualquier combinación, aunque no
     * siempre es gratis.
     * Hay que tener cuidado de no perder info, más allá del problema de recién.
     * Rust nos deja hacer algo así:
     *      some_i128 as f32 + some_f32;
     * y esto va a compilar sin problemas, pero el i128 podría ser MUCHÍSIMO más
     * grande que el f32. Entonces vamos a perder información.
     * ¡Ojo con eso!
     */ 
    
    /*
    let some_i32 = 10;
    let some_f64 = 20.2;

    let res = some_i32 as f64 + some_f64;
    println!("{}", res);
    */
    ////////////////////////////////////////////////////////////////////////////


    ///////////////////////////// SHADOWING ////////////////////////////////////
    /*
     * Shadowing nos permite definir dos o más variables distintas con el mismo
     * nombre en diferentes scopes.
     *
     */
    let var_a: i32 = 10;
    {
        println!("var_a de afuera: {}", var_a);     // 10
        let var_a: f32 = 20.2;
        println!("var_a de adentro: {}", var_a);    // 20.2
    }
    println!("var_a de afuera: {}", var_a);         // 10
    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////// CONSTANTS ///////////////////////////////////
    /*
     * Las consts son eso, constantes. Están disponibles en todo el scope
     * del programa y no cambian su valor.
     * Rust dispone de varias constantes por defecto, por ejemplo pi en f32.
     */
    println!("f64 pi = {}", std::f64::consts::PI);      // Acá usamos pi en f64
    ////////////////////////////////////////////////////////////////////////////


    //////////////////////////////// STATIC ////////////////////////////////////
    /*
     * A veces necesitamos variables globales que sean mutables.
     * Rust nos deja definirlas de la siguiente manera:
     * static mut MY_STATIC_VARIABLE: i32 = 10;
     *
     * Ahora, estas variables suelen traer un montón de problemas si no se
     * manejan con cuidado. Para no mandarnos macanas, Rust nos obliga a usar
     * la keyword "unsafe" cada vez que queremos leer o modificar una variable
     * global.
     * No puedo manipular la variables global de ninguna forma si estoy afuera
     * del scope de unsafe. ¡¡Ni siquiera imprimir el valor!!
     */
    unsafe {
        println!("variable global  = {}", MY_STATIC_VARIABLE);
        MY_STATIC_VARIABLE += 150;
        println!("variable global  = {}", MY_STATIC_VARIABLE);
    }
    ////////////////////////////////////////////////////////////////////////////




}
