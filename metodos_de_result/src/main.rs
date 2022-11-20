// Métodos de result

#[derive(Debug)]
enum ErrorDivision {
    DivisionPorCero,
    DivisionNegativos
}

fn division(numero1: i32, numero2: i32) -> Result<i32, ErrorDivision> {
    if numero2 == 0 {
        return Err(ErrorDivision::DivisionPorCero);
    }

    if numero1 < 0 || numero2 < 0 {
        return Err(ErrorDivision::DivisionNegativos);
    }

    Ok(numero1 / numero2)
}

fn main() {

    // unwrap, unwrap_or y expect

    let resultado = division(10, 2);

    let valor = resultado.expect("No es posible dividir por cero!!!");
    println!("El resultado es: {}", valor);

    /*

    match resultado {
        Ok(valor) => println!("El resultado es: {}", valor),
        Err(ErrorDivision::DivisionPorCero) => {
            println!("No es posible dividir por Cero!!!");
        }
        Err(ErrorDivision::DivisionNegativos) => {
            println!("El error es por intentar números negativos");
        }
    }

     */
}