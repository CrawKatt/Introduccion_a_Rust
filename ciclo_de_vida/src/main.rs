// Ciclo de Vida/Lifetimes
fn main() { // Bloque 1

    // Bloque limita el Scope de una variable
    let mensaje = "Hola, soy una variable del bloque main";

    let mut mensaje_dos = String::from("Hola soy una variable para préstamo");

    println!("Bloque 1: {}", mensaje);

    { // Bloque 2

        let mensaje = "Hola soy una variable del bloque 2";
        println!("Bloque 2: {}", mensaje);

        let prestamo = &mensaje_dos; // Prestamos -> la variable mensaje_dos se mueve
        // La variable mensaje_dos deja de existir para el bloque 1 y pasa a existir
        // solo en el bloque 2

        mensaje_dos = String::from("Cambio de valor");

       // println!("{}", prestamo);

        { // Bloque 3

            let mensaje = "Hola soy una variable del bloque 3";
            println!("Bloque 3: {}", mensaje);

        }

        println!("Resultado: {}", &mensaje_dos);

    }

}
