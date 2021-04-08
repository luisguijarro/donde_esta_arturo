use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
    println!("¿Donde está Arturo?");
    println!("========================================================");
    println!("Tú y Arturo estáis jugando al escondite en los jardines.");
    println!("Selecciona en que lugar vas a buscarlo.");
    println!(" 1 -> Detras de la fuente.");
    println!(" 2 -> Detras del árbol.");
    println!(" 3 -> Detras de los arbustos.");
    println!(" 4 -> Detras del banco.");
    println!(" 5 -> Detras de las rosas.");
    println!("========================================================");
    println!("");

    let escondite = rand::thread_rng().gen_range(1, 6);
        

    loop 
    {
        let mut busqueda = String::new();
        io::stdin().read_line(&mut busqueda).expect("Failed to read line");

        let busqueda: u32 = match busqueda.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Has buscado a Arturo en el sitio número {}", busqueda);
        // println!("Arturo estaba escondido en el sitio número {}", escondite);
        println!("");

        match busqueda.cmp(&escondite)
        {
            Ordering::Less => println!("Estaba escondido en un número mayor. Sigue buscando."),
            Ordering::Greater => println!("Estaba escondido en un número menor. Sigue buscando"),
            Ordering::Equal => { println!("Lo Encontraste!!!"); break; }
        }
    }
}
