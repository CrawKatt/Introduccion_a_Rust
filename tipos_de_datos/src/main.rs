fn main() {
    // str y String (próximamente en otra lección)

    // Tipos de datos
        // Rust es un lenguaje de Tipado estático por lo que
        // debemos tener muy en cuenta el tipo de dato que vayamos a utilizar en
        // nuestras variables

    // i8 (entero de 8 bits), i16 (entero de 16 bits), i32 (entero de 32 bits)
    // i64(entero de 64 bits), i128(entero de 128 bits) -> Con signo - +
    // u8, u16, u32, u64, u128 -> Sin signo

    let numero_uno : i8 = -10;
    let numero_dos : u8 = 10;

    // Char -> UTF-8

    let caracter : char = '😟'; // Con los caracteres se usa comilla simple

    // Float -> Decimales
    // f32 o f64

    let real: f32 = 12.5;

    // Bool -> Verdadero/true o Falso/false

    let resultado : bool = true; // los valores de tipo bool solo pueden contener
    // Verdadero/true o Falso/false

    println!("{} {} {} {} {}", numero_uno, numero_dos, caracter, real, resultado);
}
