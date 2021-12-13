#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
#[allow(dead_code)]

fn main() {

    // TUPLAS!!!
    // Pueden tener múltples tipos, tuplas anidadas, etc...
    let some_tuple = (2, "string".to_string(), 3.4, ("str", true));
    // tupla de tipo (i32, string, f64, (&str, bool))

    println!("Mi tupla tiene {} {}", some_tuple.0, some_tuple.1);

    // Para imprimit la tupla completa se usa {:?}
   println!("La tupla completa es {:?}", some_tuple);

   // Para acceder a lo anidado:
   // some_tuple.3.0;   Esto no compila
   // Lo podemos hacer compilar de varias maneras
   let valor_anidado = some_tuple.3 .0; // Esto sí compila
   let valor_anidado2 = (some_tuple.3).0; // Esto también compila

   let res = get_rgb();
   print!("Mi tupla de RGB: {:?}\n\tDonde:\n\t  R(red) = {}\n\t  G(green) = {}\n\t  B(blue) = {}", res, res.0, res.1, res.2);

   // Destructuring
   let (mi_r, mi_g, mi_b) = get_rgb();
   print!("Mi tupla de RGB: {:?}\n\tDonde:\n\t  R(red) = {}\n\t  G(green) = {}\n\t  B(blue) = {}", (mi_r, mi_g, mi_b), mi_r, mi_g, mi_b);

   // rgba (r, g, b, transparencia)
   let color: (u8, u8, u8, u8) = (0, 100, 150, 200);


   // Empty tuple / Unit tuple
   let empty_tuple = ();
   match color.2 {
       0..=200 => println!("Hacemos algo..."),
       _       => (),   // No hacemos nada
   }
   /* Ejemplo:
    * fn dummy() {
    *   // Acá hacemos algo
    *   15 // devolvemos un numero
    * }
    * Este proceso da error, porque espera que se devuelva una tupla vacía, no
    * un entero.
    */

}

fn get_rgb() -> (u8, u8, u8) {

    let r = 200;
    let g = 100;
    let b = 15;

    (r, g, b)
}
