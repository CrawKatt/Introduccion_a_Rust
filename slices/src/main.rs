// Slices
fn main() {
    // Un Slice es muy similar a un Arreglo/Array, la principal diferencia
    // entre un Arreglo/Array y un Slice es que a los Slices no se les conoce
    // el tamaño en tiempo de ejecución, Los Slices son almacenados en el Heap
    // mientras que los Arreglos/Arrays los cuales al conocer su longitud y
    // su tamaño son almacenados en el Stack

    // Un Slice nos permitirá prestar una sección de un Arreglo/Array, por ejemplo
    // de una Cadena/String

    let mensaje = String::from("Hola mundo, desde Rust! ");

    // Hacemos el préstamo de nuestra Cadena/String y mediante corchetes "[]"
    // seremos capaces de generar el Slice,
    let hola = &mensaje[0..4]; // Finalizamos con punto y coma ";"
    // para generar un Slice seguiremos la siguiente estructura
    // [start..end] el índice donde queremos comenzar .. el índice donde queremos terminar

    let resto_mensaje = &mensaje[5..mensaje.len() -1];

    println!("El mensaje es: {}", mensaje);

    // Comenzamos en el índice 0 y terminamos en el índice 4
    println!("El Slice es: {}",hola);

    println!("El resto del Slice es: {}", resto_mensaje);

    // Podemos omitir tanto el índice de inicio como el índice del final

    let resto_mensaje_dos = &mensaje[4..];

    println!("Resto mensaje dos: {}", resto_mensaje_dos);

    let hola_dos = &mensaje[..4];

    println!("Hola dos: {}", hola_dos);

    let mensaje_completo = &mensaje[..];

    println!("El mensaje completo es: {}", mensaje_completo);

}
