use rand::{prelude::SliceRandom, thread_rng, Rng};

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
        let option: isize = read_input().trim().parse().unwrap();
        if option == -1 {
            break;
        }

        println!("Elije el numero de la tabla");

        let num: usize = read_input().trim().parse().unwrap();

        if num > 0 && num < 11 && option == 1 {
            for (num, it, result) in generar_tabla(num) {
                println!("\t\t{num} x {it} = {result}");
            }
        }

        let mut numeros = [false; 10];

        if num > 0 && num < 11 && option == 2 {
            let mut numbs = generar_tabla(num);
            // numbs.shuffle(&mut thread_rng());

            // print!("{numbs:?}");
            let mut i = 0;
            while i < 10 {
                for _ in 0..numbs.len() {
                    let rand = rand::thread_rng().gen_range(0..10);

                    if !numeros[rand] {
                        let (num, it, result) = numbs[rand];
                        println!("{num} x {it} = ");
                        i += 1;
                        numeros[rand] = true;

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
            }
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
