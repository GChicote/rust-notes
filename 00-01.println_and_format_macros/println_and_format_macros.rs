#[derive(Debug)]
struct GabiStruct {
    nombre: String,
    apellido: String,
}

fn main() {
    /*
     * pritnln! imprime por pantalla y termina con un salto de línea
     * Ejemplos:
     */
    let a = 1.2;
    let b = 15;
    println!("Variables: {}, {}", a, b);    // Variables: 1.2, 15
    println!("Variables: {1}, {0}", a, b);  // Variables: 15, 1.2
    println!("Variables: {variable_1}, {variable_2}", variable_1 = "Chicote", variable_2 = "Gabriel");

    let data = GabiStruct {
        nombre: String::from("Gabriel"),
        apellido: String::from("Chicote"),
    };
    println!("Gabi's data is {:?}", data);  // Debug print
    println!("Gabi's data is {:#?}", data); // Pretty debug print
    let mas_data = GabiStruct {
        nombre: String::from("Mala"),
        apellido: String::from("Fama"),
    };
    println!("Gabi's data is {1:#?} and {0:#?}", data, mas_data); // Pretty debug print
    
    // format! te permite guardar la cadena formateada en una variable.
    let some_formateada = format!("Gabi's data is {0:#?} and {1:#?}", data, mas_data);
    println!("{}", some_formateada);

    // ¿Qué es el "!"?
    //
    // Es un macro, a Decalrative Macro, para ser precisos. Ni idea cual es su función.some_formateada
    // También están los Procedural Macros, que son los que tiene la pinta: #[...(...)]
    // Los declarativos se usan adentro de procesos o funciones, y los procedurals se usan sobre las
    // declaraciones de tipos.
    //
    // También uno puede crear sus propios macros. Tarea pendiente para algún momento de la vida.

}
