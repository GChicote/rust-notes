// Este es el main. Es lo mismo que en C++...
// Desde acá se maneja todo el proyecto, es el corazón de la app (?
mod geo_map;

fn main() {
    let my_fav_place = geo_map::get_hawaii_location();
    println!("Las coordenadas de mi lugar favorito son:\nlatitud: {}\nlongitud: {}", my_fav_place.lat, my_fav_place.long);

}
