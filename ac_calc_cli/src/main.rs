use std::io::{self, Write};

fn main() {
    loop {
        mostrar_menu();
        let opcion = leer_opcion();

        match opcion.as_str() {
            "1" => realizar_operacion(|a, b| Ok(a + b)),
            "2" => realizar_operacion(|a, b| Ok(a - b)),
            "3" => realizar_operacion(|a, b| Ok(a * b)),
            "4" => realizar_operacion(dividir),
            "5" => realizar_operacion(|a, b| Ok(a.powf(b))),
            "6" => realizar_operacion_unaria(calcular_raiz),
            "7" => realizar_porcentaje(calcular_porcentaje),
            "8" => {
                println!("Saliendo de la calc, Gracias!");
                break;
            }
            _ => println!("Opción inválida. Por favor, intenta de nuevo.\n"),
        }
    }
}

fn mostrar_menu() {
    println!("=== Calculadora CLI ===");
    println!("1. Sumar");
    println!("2. Restar");
    println!("3. Multiplicar");
    println!("4. Dividir");
    println!("5. Elevar");
    println!("6. RaizQ");
    println!("7. Porcentaje");
    println!("8. Salir");
    print!("Elige una opción: ");
    // Asegura que se imprima inmediatamente
    io::stdout().flush().unwrap();
}

fn leer_opcion() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error leyendo entrada");
    input.trim().to_string()
}

fn leer_numero(mensaje: &str) -> f64 {
    loop {
        print!("{}", mensaje);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error leyendo número.");

        match input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => println!("Por favor, ingresa un número válido.\n"),
        }
    }
}

fn realizar_operacion_unaria<F>(operacion: F)
where
    F: Fn(f64) -> Result<f64, String>
{
    let a = leer_numero("Ingresa el número: ");

    match operacion(a) {
        Ok(resultado) => println!("El resultado es: {}\n", resultado),
        Err(e) => print!("Error: {}\n", e),
    }
    // let resultado = operacion(a);
}

fn realizar_operacion<F>(operacion: F)
where
    F: Fn(f64, f64) -> Result<f64, String>
{
    let a = leer_numero("Ingresa el primer número: ");
    let b = leer_numero("Ingresa el segundo número: ");

    match operacion(a, b) {
        Ok(resultado) => println!("El resultado es: {}\n", resultado),
        Err(e) => println!("Error: {}\n", e),
    }
    // let resultado = operacion(a, b);
}

fn realizar_porcentaje<F>(operacion: F)
where
    F: Fn(f64, f64) -> Result<f64, String>
{
    let a = leer_numero("Ingresa el primer número: ");
    let b = leer_numero("Ingresa el segundo número: ");

    match operacion(a, b) {
        Ok(resultado) => println!("El resultado es: {}%\n", resultado),
        Err(e) => println!("Error: {}\n", e),
    }
    // let resultado = operacion(a, b);
}

fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("División por cero detectada.".to_string());
    }
    Ok(a / b)
}

fn calcular_raiz(a: f64) -> Result<f64, String> {
    if a < 0.0 {
        return Err("No se puede calcular raiz cuadrada de un número negativo.".to_string());
    }
    Ok(a.sqrt())
}

fn calcular_porcentaje(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("No se puede calcular porcentaje si el segundo número es 0.".to_string());
    }

    Ok((a / b) * 100.0)
}