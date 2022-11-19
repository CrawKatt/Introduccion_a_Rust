// Ownership

// Definimos una nueva Estructura/Struct

struct Rectangulo {
    ancho : u32,
    alto: u32
}

// Creamos una nueva función y le asignamos el parámetro "Rectangulo"
/* Descomenta este ejemplo y borra este texto para comprobar las reglas del ownership
// En la línea 13 se cumple la regla 2 del ownership
fn area(rectangulo : Rectangulo) -> u32 { // Retornará un u32
    rectangulo.ancho * rectangulo.alto
} // Una vez que un bloque finaliza, todas las variables que se hayan creado
  // en el, van a destruirse, van a desaparecer
*/

fn area(rectangulo : &Rectangulo) -> u32 {
    rectangulo.ancho * rectangulo.alto
}

fn main() {
    // El Ownership es una característica exclusiva de Rust y le permite al
    // lenguaje garantizar la seguridad de la memoria sin necesidad de utilizar
    // un recolector de basura

    // El ownership tiene 3 reglas:

    // 1.- Cada valor en Rust tiene su propio ownership
    // 2.- Solo puede existir un ownership a la vez
    // 3.- Si un ownership sale del alcance, el valor se descartará
    /* Descomenta este ejemplo y borra este texto para comprobar las reglas del ownership
     // El objeto rectangulo al pasarlo como argumento deja de existir para el scope main y ahora
     // solo existe para el scope area (línea 13)
    let rectangulo = Rectangulo { ancho:10, alto:20 }; // En la línea 31 se cumple la regla 1 del ownership
    // En la línea 31 una vez la función área haya finalizado el objeto rectangulo ya no existe en el programa
    // ya que la funcion área a finalizado
    let resultado = area(rectangulo); // En la linea 34 se cumple la regla 3 del ownership

    println!("El área de rectángulo es: {}", resultado);
    println!("El ancho y alto del rectangulo es: {} - {}", rectangulo.ancho, rectangulo.alto);
    */

    // Pasar argumentos por referencias

    let rectangulo = Rectangulo { ancho: 10, alto: 20 };

    let resultado = area(&rectangulo);

    println!("El área de rectángulo es: {}", resultado);
    println!("El ancho y alto del rectangulo es: {} - {}", rectangulo.ancho, rectangulo.alto);

}
