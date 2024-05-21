mod ant;
mod food;
mod nest;
mod world;

use std::io;
use crate::ant::Ant;
use crate::world::World;

fn main() {
    loop {
        println!("Witaj w grze MROWISKO !!!");
        println!();
        println!("Przed rozpoczęciem rozgrywki wybierz poziom trudności:");
        println!("1) Łatwy (5x5)");
        println!("2) Normalny (10x10)");
        println!("3) Trudny (20x20)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Nie udało się odczytać linii");
        let (width, height) = match input.trim() {
            "1" => (5, 5),
            "2" => (10, 10),
            "3" => (20, 20),
            _ => {
                println!("Nieprawidłowy wybór, domyślnie wybrano poziom łatwy.");
                (5, 5)
            },
        };

        let num_ants = get_limited_input("Podaj liczbę mrówek w mrowisku (proponuje 5):", width * height);
        let num_food = get_limited_input("Podaj liczbę jedzenia na mapie (proponuje 15):", width * height - 1);
        let init_food_resources = get_limited_input("Podaj zasoby jedzenia w mrowisku (jest to waluta, proponuje 50):", 500);

        let mut world = World::new(width, height, num_ants, num_food, init_food_resources);
        let mut game_over = false;
        while !game_over {
            world.display();
            println!("Cel: zdobądź {} mrówek", world.win_condition);
            println!("Zasoby jedzenia: {}", world.food_resources);
            println!("Ilość mrówek: {}", world.ants.len());

            if world.food_resources <= 0 {
                println!("Mrowisko upadło! GAME OVER");
                game_over = true;
                continue;
            }

            println!("Wybierz jedno:");
            println!("1. Kup nową mrówkę, koszt 10 zasobów jedzenia");
            println!("2. Zatrzymaj symulator");
            println!("ENTER, następna tura");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Nie udało się odczytać linii");
            match input.trim() {
                "1" => {
                    if world.food_resources >= 10 {
                        world.food_resources -= 10;
                        world.ants.push(Ant::new(world.nest.x, world.nest.y));
                        println!("Kupiono nową mrówkę");
                        if world.ants.len() >= world.win_condition {
                            println!("Gratulacje! Wygrałeś grę!");
                            let score = calculate_score(num_food, init_food_resources, world.width, world.height);
                            println!("Twój score: {}", score);
                            game_over = true;
                        }
                    } else {
                        println!("Za mało zasobów jedzenia");
                    }
                }
                "2" => {
                    println!("Symulator zatrzymany");
                    game_over = true;
                }
                _ => (),
            }

            if !game_over {
                world.update();
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        }

        println!("Czy chcesz zagrać ponownie? (tak/nie)");
        input.clear();
        io::stdin().read_line(&mut input).expect("Nie udało się odczytać linii");
        if input.trim().to_lowercase() != "tak" {
            break;
        }
    }
}

fn get_limited_input(prompt: &str, max: usize) -> usize {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Nie udało się odczytać linii");
        match input.trim().parse::<usize>() {
            Ok(i) if i > 0 && i <= max => return i,
            _ => println!("Błąd: Proszę wprowadzić liczbę większą od zera i nie większą niż {}.", max),
        }
    }
}

fn calculate_score(num_food: usize, init_food_resources: usize, world_width: usize, world_height: usize) -> f64 {
    (world_width * world_height) as f64 * 1000.0 / ((num_food + init_food_resources) as f64)
}

