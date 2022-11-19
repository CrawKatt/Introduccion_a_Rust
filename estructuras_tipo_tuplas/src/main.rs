// Algo importante a mencionar es que Rust tiene
// Estructuras/Struct de tipo Tupla, es decir, este tipo de Estructuras/Struct
// en lugar de poseer atributos, poseerá valores

// Para poder definir una Estructura/Struct de tipo Tupla
// utilizamos la palabra reservada Struct seguido del nombre de
// nuestra Estructura/Struct

// En una Estructura/Struct convencional, haríamos uso de un juego de llaves "{}"
// y dentro de ellos colocaríamos los atributos para la Estructura/Struct

// Sin embargo, para una Estructura/Struct de tipo Tupla
// haremos uso de paréntesis "()" y dentro de los paréntesis vamos a definir
// la cantidad de elementos que la Tupla podrá almacenar asi como el
// Tipo de Dato para cada elemento

// Para este ejemplo haremos uso de #[derive(Debug)] sobre
// nuestra Estructura/Struct para poder imprimir nuestra
// Estructura/Struct de Tipo Tupla
#[derive(Debug)]
struct Color(u32, u32, u32); //RGB (Red, Green, Blue)
// No vamos a definir atributos, vamos a definir Tipos de Datos


fn main() {

    let black = Color(0,0,0);
    let white = Color(255,255,255);
    println!("El color es: {:?}", black); // Utilizamos {:?} ya que estamos imprimiendo una Tupla
    println!("El color es: {:?}", white);

    // Al tratarse de una Estructura/Struct de Tipo Tupla, pdoemos
    // tratar a nuestros objetos como una Tupla

    // En este caso necesitamos que nuestra variable sea mutable
    let mut custome_color = Color(187,62,104);
    custome_color.1 = custome_color.1 + 10; // Accedemos a su indice numero 1 e
    // incrementamos su valor en 10
    println!("El color es: {:?}", custome_color);
}
