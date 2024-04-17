use rand::{prelude::SliceRandom, thread_rng};

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    loop {
        println!("Opcion 1: Imprimir tabla con numero. \nOpcion 2: Adivinar el numero. \nOpcion -1: Salir");
        let option = read_input().trim().parse::<isize>();
        match option {
            Ok(-1) => break,
            Ok(1 | 2) => loop {
                println!("Elije el numero para generar la tabla");
                let num: isize = read_input().trim().parse().unwrap();
                match (num > 0, num < isize::MAX / 10, option.as_ref().unwrap()) {
                    (true, true, 1) => imprimir_tabla(num.try_into().unwrap()),
                    (true, true, 2) => adivinar_tabla(num.try_into().unwrap()),
                    (false, _, _) => {
                        println!("Introduce un numero positivo");
                        continue;
                    }
                    (_, false, _) => {
                        println!("El numero es muy grande");
                        continue;
                    }
                    _ => continue,
                }
            },
            _ => {
                println!("Introduce una opcion valida");
                continue;
            }
        }
    }
}

fn imprimir_tabla(num: usize) {
    for (num, it, result) in generar_tabla(num) {
        println!("\t\t{num} x {it} = {result}");
    }
}

fn adivinar_tabla(num: usize) {
    let mut numbs = generar_tabla(num);
    numbs.shuffle(&mut thread_rng());

    for (num, it, result) in numbs {
        println!("{num} x {it} = ");

        for _ in 0..3 {
            let user_try: usize = read_input().trim().parse().unwrap();

            if result == user_try {
                println!("Correcto! :)");
                break;
            }

            println!("Intenta de nuevo :(");
        }
    }
}

fn generar_tabla(num: usize) -> Vec<(usize, usize, usize)> {
    let mut numeros: Vec<(usize, usize, usize)> = Vec::new();

    for i in 1..=10 {
        numeros.push((num, i, i * num));
    }

    numeros
}
