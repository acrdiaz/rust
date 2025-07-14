use std::io::{self, Write};

fn main() {
    loop {
        mostrar_menu();
        let opcion = leer_opcion();

        match opcion.as_str() {
            "1" => realizar_operacion(|a, b| a + b),
            "2" => realizar_operacion(|a, b| a - b),
            "3" => realizar_operacion(|a, b| a * b),
            "4" => realizar_operacion(dividir),
            "5" => {
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
    println!("5. Salir");
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

fn realizar_operacion<F>(operacion: F)
where
    F: Fn(f64, f64) -> f64,
{
    let a = leer_numero("Ingresa el primer número: ");
    let b = leer_numero("Ingresa el segundo número: ");

    let resultado = operacion(a, b);
    println!("El resultado es: {}\n", resultado);
}

fn dividir(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("División por cero detectada!");
    }
    a / b
}