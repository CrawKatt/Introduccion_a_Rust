// Ciclo loop
fn main() {
    // El Ciclo loop genera un ciclo infinito

    loop { // Utilizamos la palabra reservada loop seguido de un bloque
           // a través de un juego de llaves "{}"
        println!("Hola, nos encontramos dentro de un ciclo infinito");
        // Para cancelar el ciclo infinito en ejecución utilizamos
        // CTRL + C
        break;
    }
    // Podemos terminar un ciclo loop mediante la palabra reservada break
    // Lo que hará break es terminar el ciclo de manera abrupta

    // Ejemplo

    // Creamos una nueva variable
    let mut contador  = 0;

    // Creamos el ciclo loop e introducimos nuestra variable dentro del bloque
    loop {
        contador += 1; // += 1 suma una repetición a nuestro ciclo

        println!("Contador: {}", contador);

        // Para que nuestro ciclo loop no sea un
        // ciclo infinito vamos a condicionar el ciclo para que se detenga
        // cuando sea mayor que 10

        if contador >= 10 {
            println!("Numero de repeticiones: 10");

            break;
            // El ciclo se detiene en 10
            // ya que es mayor que 10 por lo que la condición se cumple
        }
    }
}
