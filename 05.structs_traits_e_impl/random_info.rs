/* Acá se definieron algunas funciones, pero también pudimos 
 * definir funciones en struct.rs.
 */

// Traits
pub trait SomeTrait {
    fn is_valid(&self) -> bool;
    //fn get_the_better_one(&self, some_other: &self) -> Self;
}

#[derive(Debug)]
// Primero definamos nuestra estructura
pub struct RandomInfo {
    pub call_count: i64,
    pub some_bool: bool,
    pub some_int: i64,
}

/* Ahora definmos la implementación de las funciones
 * Self (mayúscula) -> significa que es el tipo
 * self (minúscula) -> significa que representa algún dato
 *
 * Batalla de operadores: :: vs .
 * Cuando se está leyendo o escribiendo una variable (usando la keyword &self)
 * se usa .
 * Cuando no se tiene la keyword &self, indica que la función debería estar
 * disponible para el tipo en sí, y se usa ::
 */
impl RandomInfo {
    pub fn new(a: bool) -> Self {
        Self {
            call_count: 0,
            some_bool: !a,
            some_int: 8,
        }
    }

    pub fn is_smaller(&mut self, compare_to: i64) -> bool {
        self.call_count++;
        self.some_int < compare_to
    }

}


/* Implementación de SomeTrait
 */
impl SomeTrait for RandomInfo {
    fn is_valid(&self) -> bool {
        self.some_bool
    }
}
