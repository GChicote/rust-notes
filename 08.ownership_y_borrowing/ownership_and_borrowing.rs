
#[allow(unused_variables)]
fn main() {

    ////////////////////////////////////////////////////////////////////////////
    // Variables en el stack
    //let stack_i8: i8 = 10;
    //let stack_f32: f32 = 10.;
    //let stack_bool: bool = true;
    //let stac_char: char = 'j';
    //
    // Variables en el heap
    //let heap_vector: Vec<i8> = Vec::new();  // Vec![1,2,3,4,5] lo inicializa en el acto
    //let heap_string: String = String::from("Hooray");
    //let heap_i8: Box<i8> = Box::new(30);
    //
    // Las cosas del stack se pueden sopiar de la siguiente manera:
    // Esto está permitido porque copiar cosas en el stack es rápido
    //let stack_i8_2 = stack_i8;
    //println!("{}", stack_i8);
    //println!("{}", stack_i8_2);
    //
    // Pero copiar cosas que están en el heap no es rápido en absoluto...
    // mirá si tenés un vector de mil millones de elementos, eso va a ser todo
    // menos rápido. Para eso, Rust implementa los conceptos de ownership y 
    // borrowing:
    //   - En rust, cada "pedazo" de data en memoria tiene un dueño (la variable
    //     que lo contiene).
    //   - Sólo puede haber un dueño a la vez.
    // El siguiente código va a dar un error al compilar, ¿por qué? Porque al 
    // asignar heap_i8 a heap_i8_2, lo que estamos haciendo es transferir la 
    // "propiedad", el ownership de esa data de heap_i8 a heap_i8_2, la memoria
    // quedó intacta. Así, luego de la transferencia, heap_i8 ya no apunta a 
    // una porción de memoria, y de ahí el error.
    //
    // let heap_i8_2 = heap_i8;
    // println!("{}", heap_i8);
    // println!("{}", heap_i8_2);
    //
    // Para solucionar esto y poder hacer una copia de heap_i8 y guardarla en
    // heap_i8_2. Podemos hacer un par de cosas, podemos:
    //      1 - Pedir prestado el ownership (hacer un borrow).
    //      2 - Copiar el contenido en memoria
    // 1 - Ésto se hace con el &, nada nuevo por acá.
    // 2 - Y ésto se hace con el método .clone():
    //      let heap_i8_2 = heap_i8.clone();
    ////////////////////////////////////////////////////////////////////////////
    
    ////////////////////////////////////////////////////////////////////////////
    //let heap_i8_2 = heap_i8.clone();
    //println!("{}", heap_i8);
    //println!("{}", heap_i8_2);
    //
    //let stack_f64: f64 = 1.;
    //let heap_f64: Box<f64> = Box::new(2.);
    //
    // Cuando pasamos una variable del stack por parámetro, en realidad se 
    // está pasando por copia y todos felices.
    // También se puede pasar por referencia, eh, ningún problema. Rust prefiere
    // hacer copias para que no hagamos lio con el dato original.
    //stack_proc(stack_f64);
    //println!("En main stack {}", stack_f64);
    //
    // Pero cuando pasamos una variable del heap por parámetro, lo que pasa es
    // que se transfiere el ownership a esa función. Esto tiene un problema.
    // Como la memoria del heap se limpia cuando el dueño de cierto dato sale
    // de scope, la variable que pasamos se va a perder cuando se termine de
    // ejecutar la función, entonces "heap_f64" no tiene datos asociados.
    // Podemos hacer un par de cosas para solucionar esto:
    //   1 - Hacer que la función devuelva el dato y guardar eso en una variable
    //   2 - Hacer un clone cuando lo pasamos por parámtro
    //   3 - Pasar por referencia.
    //
    // 1 es: 
    //      heap_f64 = heap_proc(heap_f64); 
    //      Esto es medio una fiaca, no? Tener que hacerlo cada vez que llamamos
    //      a una función no es negocio.
    //      Mirá si la función toma más de un parámtro. ¿Qué hacemos? ¿Devol-
    //      vemos una tupla? Mm... medio dudoso este método, ¿no?
    //      Además tenemos que hacer que la variable sea mutable la primera vez
    //      que la declaramos.
    //
    // 2 es:
    //      heap_proc(heap_f64.clone());
    //      Esto sirve, pero puede ser demasiado costoso, de nuevo, mirá si
    //      tenés un array de mil millones de elementos... na, dejá
    //
    // 3 es:
    //      heap_proc(&heap_f64);
    //      Esto... Esto me gusta. Hagamos esto
    //
    //heap_proc(&heap_f64);
    //println!("En main {}", heap_f64);
    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    //let some_string: String = String::from("Hooray!");
    // Las strings están siempre en el heap proque son mutables
    //
    //let some_str: &str = "eeeeeeeeee";
    // &str es un puntero
    //
    //some_proc(&some_string, some_str);
    // Si queremos volver a usar some_string después de llamar a la función, 
    // debemos pasarla por referencia para que no se pierda el ownership.
    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    //let var_a = String::from("Hooray");
    //let var_b = &var_a;
    //let var_c = &var_a;
    // Esto está perfecto porque el único owner es var_a.
    // Si hacemos que var_a sea mut, ahí PODRÍAMOS llegar a tener problemas, 
    // pero tal vez no. Por las dudas, el compilador no nos deja.
    //
    // let mut var_a = String::from("Hooray");
    // let var_b = &var_a;
    // let var_c = &var_a;
    // println!("{} {} {}", var_a, var_b, var_c);
    // var
    //
    // Si modificamos var_a antes de usar var_b o var_c, estamos al horno.
    // Pero si la modificamos después de usarlas, no pasa nada.
    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    //
    //let var_a = String::from("Hooray!");
    //let var_b = String::from("Partner");
    //
    //let mass_data: Vec<&String> = vec![&var_a, &var_b]; // Podrían ser millones
    //
    //println!("{}", heavy_calc(mass_data));
    //
    // Como los datos de var_a y vas_b son inmutables, todavía puedo usar esas
    // variables
    //println!("{} {} {:?}", var_a, var_b, mass_data);
    //
    // Todo lo de la función también se podría haber hecho con &str.
    //
    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    //let var_1 = GabiStruct { a:9, b:10. };
    //some_procedure(var_1);
    //println!("{:?}", var_1);
    // Acá tenemos un error de compilación.
    // Una vez que pasamos var_1 a la función, ya cambió de dueño y al terminar
    // ésta, termina también el scope de validez para var_1, por ende se borra.
    // Para solucionar eso, lo más eficiente sería pasar la variable por 
    // referencia. Peeeeero ¿Y si queremos una copia? Bueno, podríamos clonarla
    // haciendo:
    //          some_procedure(var_1.clone());
    // ¿No? Mm... no tan rápido, amigo (?
    // Clone no está definido para structs. No desesperéish, amigo mio.
    // Lo único que hay que hacer para poder usar clone, es decirle que soporte
    // clone() en la anotación ( #[...(...)] )... sí, así de simple.
    // Nos quedaría así:
    //          #[derive(Debug, Clone)]
    // Ésto es un macro.
    // También se puede usar el macro Copy que te ahorra escribir el método
    // .clone().
    // Copy necesita que Clone también esté. No importa el orden en el que estén
    //
    // Las Strings no se pueden copiar. Vas a tener que pasarlo por referencia
    //
    ////////////////////////////////////////////////////////////////////////////
}

#[derive(Debug, Copy, Clone)]
struct GabiStruct {
    a: i32,
    b: f64,
}

fn some_procedure(param_a: GabiStruct) {
    println!("{:?}", param_a);
}


/*
fn heavy_calc(_param: &Vec<&String>) -> i64 {
    // Do some shit
    10
}
*/


/*
fn stack_proc(mut param: f64) {
    param += 9.;
    println!("En stack_proc con el parámetro {}", param);
}

*/

/*
fn heap_proc(param: &Box<f64>) {
    println!("En heap_proc con el parámetro {}", param);
}
*/


/*
fn some_proc(param_a: &String, param_b: &str) {
    println!("{} {}", param_a, param_b);
}
*/
