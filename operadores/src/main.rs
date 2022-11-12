// Operadores en Rust
fn main() {
    // Definimos nuestras variables
    let numero_uno : i32 = 10;
    let numero_dos : i32 = 200;

    // + - / * % (Suma Resta División Multiplicación División con Residuo/Resto)
    // Suma "+"
    let resultado_suma : i32 = numero_uno + numero_dos;

    // Imprimimos todos los ejemplos de operadores
    println!("El resultado es: {}", resultado_suma);

    // Multiplicación "*"
    let _resultado_mul : i32 = numero_uno + numero_dos;
    // Para omitir Warns (advertencias) en el compilador de Rust por
    // variables sin uso podemos utilizar _ antes del nombre de nuestra variable
    // para que no salte la advertencia por variables sin uso

    // También podemos usar valores de tipo Bool (true/false)
    // Operadores Relacionales

    // mayor que ">"
    let resultado_mayor : bool = resultado_suma > 100;

    println!("El resultado es: {}", resultado_mayor);

    // menor que "<"
    let resultado_menor : bool = resultado_suma < 100;

    println!("El resultado es: {}", resultado_menor);

    // mayor o igual ">="
    let resultado_mayor_igual : bool = resultado_suma >= 100;

    println!("El resultado es: {}", resultado_mayor_igual);

    // menor o igual "<="
    let resultado_menor_igual : bool = resultado_suma <= 100;

    println!("El resultado es: {}", resultado_menor_igual);

    // igual "=="
    let resultado_igual : bool = resultado_suma == 2000;

    println!("El resultado es: {}", resultado_igual);

    // diferente "!="
    let resultado_diferente : bool = resultado_suma != 2000;

    println!("El resultado es: {}", resultado_diferente);

    //Operadores Lógicos // && (and) || (or)

    // || (or)
    let resultado_or : bool = 20 + 10 > 100 || true; // Retorna true ya
    // que en la segunda condición es true
    // || (or) Retorna true siempre
    // que una de las condiciones sea true

    println!("El resultado es: {}", resultado_or);

    // && (and)
    let resultado_and : bool = 20 + 10 > 100 && true; // Todas las condiciones
    // deben ser true para retornar true
    // En este caso, retornará false, ya que
    // no todas las condiciones son verdaderas

    println!("El resultado es: {}", resultado_and);

}
