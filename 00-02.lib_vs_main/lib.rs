// Esta es una librería. Acá se definen funciones y se exportan en otros 
// archivos.
// No tiene función main porque desde acá no es ejecutable, sino utilizable por
// otros archivos.
// En resumen... es un módulo.
mod geo_map;

pub fn funcion_que_hace_algo() {
    let my_fav_place = geo_map::get_hawaii_location();

    println!(
        "Las coordenadas de mi lugar favorito son:\nlatitud: {}\nlongitud: {}",
        my_fav_place.lat, my_fav_place.long
    );

}
