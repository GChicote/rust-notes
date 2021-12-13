#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    /// ENTEROS:
    /* Hay enteros de 8, 16 ,32 ,64, 128 bits, todos con y sin signo.
     * Si no se especifica el tamaño, se asume de 32 bits.
     *
     * Ej: i8 -> entero de 8 bits con signo.
     *     u8 -> entero de 8 bits sin signo.
     *
     * Enteros de tamaño de la arquitectura del sistema
     * El compilador detecta la arquitectura del sistema y le asigna dicho
     * tamaño de memoria.
     * Ej: let num: isize = 10; // Le asigna el tipo de 64b
     *     let num: usize = 10; // Ídem, pero sin signo
     */

    /// FLOTANTES:
    /* Hay solo de 32 y 64 bits. TIENEN que tener un . en el número, sino
     * el compilador se queja.
     * Si no se especifica el tamaño, se asume de 64 bits.
     *
     * Ej: let n: f32 = 5;      // ERROR.
     *     let n: f64 = 125.;   // Bien, éste float de 64 bits sí compila
     *
     */

    /// CHARS:
    /* Los char se definen entre ', no ".
     * Se guardan como u8.
     * Ej: let c: char = "r";   // ERROR, no compila.
     *     let c: char = 'r';   // 10 puntos!
     */

    /// STRINGS:
    /* Hay dos tipos: &str y String. Se diferencian en como se guardan en
     * memoria, y por ende, los usos que se les da.
     * Ambos son grupos de char (u8).
     *  - String: se guardan en el heap, lo que permite que sean mutables.
     *  - &str: (Slices) son inmutables (¿Casi siempre?).
     *          "Often allocated on the stack, sometimes a heap reference,
     *          sometimes embedded in the code".
     * Se puede ir de una a otra sin problemas.
     * 
     * Ej: let ejemplo_str: &str = "Hola";
     *     let ejemplo_string: String = String::from("Hola");
     *     
     *     let string_from_str: String = ejemplo_str.to_string();
     *     let otra_string_from_str = String::from(ejemplo_str);
     *
     *     let str_from_string: &str = &ejemplo_string;
     * 
     * Concatenar strings:
     * let res = "Gabriel" + "Chicote";             // ERROR
     * let res = ["Gabriel", "Chicote"].concat();   // Compila
     * let con_format = format!("{} {}", "Gabriel", "Chicote");
     *
     * let string_mas_str = ejemplo_string + ejemplo_str
     * // Lo de arriba anda sólamente si la string está antes que la slice
     *
     * let mut mut_string = String::new(); // El compilador infiere que es de
     *                                        tipo string por el generador.
     *
     * Para agregar hay varios métodos.
     * mut_string.push_str(ejemplo_str);
     * mut_string.push_str("Algo para agregar");
     * mut_string.push('c'); 
     *
     * let a = String::from("a");
     * let b = String::from("b");
     * let combinacion = a + &b + &mut_string;
     *
     *
     * Substrings:
     * let str_from_substr: &str = &ejemplo_str[n..m]; // dede n hasta m-1
     * let str_from_substr: &str = &ejemplo_str[..m];  // de inicio hasta m-1
     * let str_from_substr: &str = &ejemplo_str[n..];  // de n hasta el final
     *
     * Se puede hacer lo mismo con ejemplo_string.
     *
     * Si se quiere que incluya también el último elemento del rango:
     * let str_from_substr: &str = &ejemplo_string[n..=m];// dede n hasta m-1
     * let str_from_substr: &str = &ejemplo_string[..=m];// de inicio hasta m-1
     *
     * Para acceder a un elemento de la cadena (tanto strings como &str):
     * let char_by_index = &ejemplo_str[i];             // No compila
     * let char_by_index = &ejemplo_str.chars().nth(i); // Esto sí
     * 
     * Pero esto puede indefinirse y tirar un error. Para manejar ese error:
     * match char_by_index {
     *      Some(c) => println!("Found a char {}", c),
     *      None    => {}
     * }
     *
     * Una manera más corta:
     * if let Some(c) = ejemplo_str.chars().nth(2) {
     *      println!("Found a char {}", c);
     * }
     *
     */
    
}
