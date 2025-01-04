use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("O número secreto é: {secret_number}");

    loop{

        println!("Por favor, insira seu palpite");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falhou ao ler a linha");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite foi: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Muito Baixo!"),
            Ordering::Greater => println!("Muito Alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }

    }

    
}
