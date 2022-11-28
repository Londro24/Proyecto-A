use std::io::stdin;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;


#[derive(Default)]
struct Producto{
    codigo: String,
    nombre: String,
    costo: String,
    venta: String,
    stock: String
}


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


fn open_file_to_append(path: &Path) -> File{
    open_file(path);
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file: File = match binding.open(path){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}


fn read_file(mut file: &File) -> String {
    let mut text: String = String::new();
    file.read_to_string(&mut text).unwrap();
    return text
}


fn create_blank_file(path: &Path){
    let _file: File = File::create(path).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}


fn open_file(path: &Path) -> String{
    let mut text: String = "".to_string();
    if Path::new(path).exists(){
        let file: File = match File::open(&path){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        text = read_file(&file);
    } else {
        create_blank_file(path);
    }
    return text
}


fn cambiar_inventario(inventario: &Path, entrada: String) -> Producto {
    let mut producto: Producto = Default::default();
    let mut text_inventario: String = open_file(inventario);
    let mut flie_inventario: File = open_file_to_append(inventario);

    for a in text_inventario.split("\n") {
        let mut contador = 0;
        for b in a.split(",") {
            if entrada.trim() != b && contador == 0{
                break
            }
            match contador {
                0 => producto.codigo = b.to_string(),
                1 => producto.nombre = b.to_string(),
                2 => producto.costo = b.to_string(),
                3 => producto.venta = b.to_string(),
                4 => producto.stock = b.to_string(),
                _ => panic!("AAAAAAAAAAAAAAAA")
            }
            contador += 1;
        }
    }

    return producto;
}


fn fin_venta(suma: u32) {
    println!("Total: ${}", suma);
    let mut monto: String = String::new();
    let mut monto_u32: u32 = 0;
    loop {
        println!("Ingrese el monto:");
        stdin().read_line(&mut monto).unwrap();
        if !is_entero_positivo(&monto) || monto.trim() == "".to_string() {
            println!("\nMonto no válido\n");
            monto = "".to_string();
            continue
        }
        monto_u32 = monto.trim().parse().unwrap();
        if monto_u32 < suma {
            println!("\nMonto insuficiente\n");
            monto = "".to_string();
            continue
        }
        break
    }
    let vuelto: u32 = monto_u32 - suma;
    println!("Vuelto: ${}", vuelto);
}


fn vender(finanzas: &Path, inventario: &Path) {
    let mut suma = 0;

    loop {
        let mut entrada: String = String::new();
        stdin().read_line(&mut entrada).unwrap();

        if entrada.trim() == "0" {
            fin_venta(suma);
            break
        }

        let mut text_finanzas: String =  open_file(finanzas);
        let mut file_finanzas: File = open_file_to_append(finanzas);
        
        let producto: Producto = cambiar_inventario(inventario, entrada);
     }
}


fn ingresar(finanzas: &Path, inventario: &Path) {
    print!("EEE")
}


fn consultar(finanzas: &Path, inventario: &Path) {
    print!("III")
}


fn editar(finanzas: &Path, inventario: &Path) {
    print!("OOO")
}


fn agregar_nuevo(finanzas: &Path, inventario: &Path) {
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

        if !is_entero_positivo(&entrada) || entrada.trim() == "".to_string() || entrada.trim().len() > 2 {
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
    loop {
        let opcion: u32 = menu();
        match opcion {
            1 => vender(finanzas, inventario),
            2 => ingresar(finanzas, inventario),
            3 => consultar(finanzas, inventario),
            4 => editar(finanzas, inventario),
            5 => agregar_nuevo(finanzas, inventario),
            _ => break
        }
    }

}
