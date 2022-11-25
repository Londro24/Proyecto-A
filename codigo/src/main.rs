use std::io::stdin;
use std::path::Path;


fn is_entero_positivo(numero: &str) -> bool {
    for digit in numero.to_string().trim().chars(){
        if digit.is_numeric(){
            continue
        } else {
            return false
        }
    }
    return true
}


fn vender(finanzas: &Path, inventario: &Path, clientes: &Path) {
    print!("AAA")
}


fn ingresar(finanzas: &Path, inventario: &Path, clientes: &Path) {
    print!("EEE")
}


fn consultar(finanzas: &Path, inventario: &Path, clientes: &Path) {
    print!("III")
}


fn editar(finanzas: &Path, inventario: &Path, clientes: &Path) {
    print!("OOO")
}


fn agregar_nuevo(finanzas: &Path, inventario: &Path, clientes: &Path) {
    print!("UUU")
}


fn menu() -> u32 {
    let mut entrada: String = String::new();
    loop {
        println!("\nElija opción:");
        println!("    (1) Vender.");
        println!("    (2) Ingresar al inventario.");
        println!("    (3) Consultar producto.");
        println!("    (4) Editar productos.");
        println!("    (5) Agregar un producto nuevo.");
        println!("    (0) Salir del programa.");
        stdin().read_line(&mut entrada).unwrap();

        if !is_entero_positivo(&entrada) || entrada.trim() == "".to_string() {
            println!("\nIntentelo denuevo\n");
            entrada = "".to_string();
            continue
        }

        match entrada.trim().parse().unwrap() {
            0|1|2|3|4|5 => break,
            _ => entrada = "".to_string()
        }
        println!("\nIntentelo denuevo\n");
        continue
    }   
    let num: u32 = entrada.trim().parse().unwrap();
    return num
}


fn main() {
    let finanzas: &Path = Path::new("Finanzas.csv");
    let inventario: &Path = Path::new("inventario.csv");
    let clientes: &Path = Path::new("clientes.csv");

    loop {
        let opcion: u32 = menu();
        match opcion {
            1 => vender(finanzas, inventario, clientes),
            2 => ingresar(finanzas, inventario, clientes),
            3 => consultar(finanzas, inventario, clientes),
            4 => editar(finanzas, inventario, clientes),
            5 => agregar_nuevo(finanzas, inventario, clientes),
            _ => break
        }
    }
}
