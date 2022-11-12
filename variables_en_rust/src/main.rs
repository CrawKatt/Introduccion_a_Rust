//Variables en Rust
fn main() {
// Palabra Reservada let seguido del nombre de la variable
    // En Rust, por defecto, las variables son inmutables, si deseamos hacer que una variable
    // tome diferentes valores, debemos indicarle a Rust que esa variable va a ser mutable
    // utilizando mut después de let.
    // Ejemplo: let mut variable = valor;
    let mut numero_uno= 10; // Valor variable y terminar en ;
    numero_uno = 100;
    // Utilizar mut solo cuando sea necesario que la variable sea mutable

  // Tipos de datos en Rust
  // Uso: let variable : tipo de dato = valor;
    //Tipo i32 = numero entero
    let numero_dos : i32 = 10;

  // Suma de variables en Rust
    let resultado = numero_uno + numero_dos;

  // Imprimimos el resultado usando {} seguido de una , al final de las "" y
    println!("El resultado es ({} + {} = {})",numero_uno, numero_dos, resultado); // Escribiendo el nombre de la variable (finalizar con ; )
    // Se pueden imprimir multiples variables con multiples {} seguido de una coma al escribir el nombre de las variables
}
