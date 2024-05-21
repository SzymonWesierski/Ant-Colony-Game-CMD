use crate::ant::Ant;
use crate::food::Food;
use crate::nest::Nest;
use rand::Rng;

#[derive(Debug)]
pub struct World {
    pub width: usize,
    pub height: usize,
    pub ants: Vec<Ant>,
    pub food: Vec<Food>,
    pub nest: Nest,
    pub food_resources: usize,
    pub turn: usize,
    pub win_condition: usize,
}

impl World {
    pub fn new(width: usize, height: usize, num_ants: usize, num_food: usize, init_food_resources: usize) -> World {
        let nest = Nest::new(width / 2, height / 2);

        let food_resources = init_food_resources;

        let ants = (0..num_ants).map(|_| Ant::new(nest.x, nest.y)).collect();

        let mut rng = rand::thread_rng();
        let food = (0..num_food)
            .map(|_| Food::new(rng.gen_range(0..width), rng.gen_range(0..height)))
            .collect();

        let win_condition = num_ants * 4;

        World {
            width,
            height,
            ants,
            food,
            nest,
            food_resources,
            turn: 0,
            win_condition,
        }
    }

    pub fn display(&self) {
        println!("{}{}", "-".repeat(self.width * 14), "-");
        // Wyświetl numer kolumn
        print!("|         |");
        for x in 0..self.width {
            print!(" {:^9}  |", x);
        }
        println!();
        println!("{}{}", "-".repeat(self.width * 14), "-");
    
        for y in 0..self.height {
            // Wyświetl numer wiersza
            print!("|{:^9}|", y);
            for x in 0..self.width {
                let mut field_to_print = String::new();

                if self.nest.x == x && self.nest.y == y {
                    field_to_print += "🏠";
                } 
                
                if self.ants.iter().any(|ant| ant.x == x && ant.y == y) {
                    let ants_at_this_position = self.ants.iter().filter(|ant| ant.x == x && ant.y == y);

                    if self.turn > 0{
                        for ant in ants_at_this_position {
                            if ant.carrying_food {
                                field_to_print += "🐜🍃";
                            } else {
                            field_to_print += "🐜";
                            }
                        }
                    }else{
                        field_to_print += &format!("  {}x 🐜   ", self.ants.len());
                    }
                    
                }
                else if self.food.iter().any(|food| food.x == x && food.y == y) {
                    field_to_print += "🍃";
                }

                let mut chars_in_field_to_print = field_to_print.chars().count();
                chars_in_field_to_print *= 2;

                while chars_in_field_to_print < 12 {
                    field_to_print += " ";
                    chars_in_field_to_print += 1;
                }
            
                print!("{}", field_to_print);

                print!("|");
            }
            println!("\n{}", "-".repeat(self.width * 14));
        }
        
    }

    pub fn is_free_space(&self, x: usize, y: usize) -> bool {
        let is_free_from_ants = !self.ants.iter().any(|ant| ant.x == x && ant.y == y);
        let is_free_from_food = !self.food.iter().any(|food| food.x == x && food.y == y);
        let is_not_nest = !(self.nest.x == x && self.nest.y == y);

        is_free_from_ants && is_free_from_food && is_not_nest
    }

    pub fn update(&mut self) {
        self.turn += 1;

        if self.food_resources > 0 {
            self.food_resources -= 1;
        }

        println!("--- Turn {} ---", self.turn);

        if self.turn % 3 == 0 {
            let mut rng = rand::thread_rng();
            let mut x = rng.gen_range(0..self.width);
            let mut y = rng.gen_range(0..self.height);

            while !self.is_free_space(x, y) {
                x = rng.gen_range(0..self.width);
                y = rng.gen_range(0..self.height);
            }

            self.food.push(Food::new(x, y));
            println!("Na świecie wylądował liść w polu ({}, {})", x, y);
        }

        for ant in &mut self.ants {
            ant.move_ant((self.width, self.height));
    
            if !ant.carrying_food {
                for (i, food) in self.food.iter().enumerate() {
                    if ant.x == food.x && ant.y == food.y {
                        ant.carrying_food = true;
                        ant.moves.pop();
                        println!("Mrówka zebrała liść w polu ({}, {})", ant.x, ant.y);
                        self.food.remove(i);
                        break;
                    }
                }
                
            }
        
            if ant.carrying_food && ant.x == self.nest.x && ant.y == self.nest.y {
                ant.carrying_food = false;
                self.food_resources += 10;
                ant.moves.clear();
                ant.moves.push((self.nest.x, self.nest.y));
                println!("Mrówka przyniosła liść do mrowiska");
            }
        }
    }
}