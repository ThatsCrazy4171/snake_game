use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::fmt;
use std::io;

#[derive(PartialEq, Debug)]
enum State {
    Snake,
    BronzeSnake,
    SilverSnake,
    GoldSnake,
    DiamondSnake,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self)
    }
}

enum Enemies {
    Frog,
    Lizard,
    Mouse,
    Bird,
}

enum Biome {
    Swamp,
    Prarie,
    Mountain,
}

struct Snake {
    state: State,
}

impl Snake {
    fn new() -> Self {
        Self {
            state: State::Snake,
        }
    }
    fn collect(&mut self, power: Enemies) {
        match (&self.state, power) {
            (State::Snake, Enemies::Frog) => self.state = State::BronzeSnake,
            (State::BronzeSnake, Enemies::Frog) => self.state = State::BronzeSnake,
            (State::SilverSnake, Enemies::Frog) => self.state = State::SilverSnake,
            (State::GoldSnake, Enemies::Frog) => self.state = State::GoldSnake,
            (State::DiamondSnake, Enemies::Frog) => self.state = State::DiamondSnake,

            (_, Enemies::Lizard) => self.state = State::SilverSnake,
            (_, Enemies::Mouse) => self.state = State::GoldSnake,
            (_, Enemies::Bird) => self.state = State::DiamondSnake,
        }
    }
}

struct Location {
    biome: Biome,
    capacity: u32,

    frogs: u32,
    lizards: u32,
    mice: u32,
    birds: u32,

    frog_chance: f32,
    lizard_chance: f32,
    mice_chance: f32,
    bird_chance: f32,

    population: u32,
}

impl Location {
    fn swamp() -> Self {
        Self {
            biome: Biome::Swamp,
            capacity: 5,
            frogs: 0,
            lizards: 0,
            mice: 0,
            birds: 0,
            frog_chance: 0.50,
            lizard_chance: 0.35,
            mice_chance: 0.10,
            bird_chance: 0.05,
            population: 0,
        }
    }
    fn prarie() -> Self {
        Self {
            biome: Biome::Prarie,
            capacity: 8,
            frogs: 0,
            lizards: 0,
            mice: 0,
            birds: 0,
            frog_chance: 0.,
            lizard_chance: 0.0,
            mice_chance: 0.0,
            bird_chance: 0.0,
            population: 0,
        }
    }

    fn spawn_enemies(&mut self) {
        match &self.biome {
            Biome::Swamp => {
                let choices = [
                    (Enemies::Frog, self.frog_chance),
                    (Enemies::Lizard, self.lizard_chance),
                    (Enemies::Mouse, self.mice_chance),
                    (Enemies::Bird, self.bird_chance),
                ];
                let dist = WeightedIndex::new(choices.iter().map(|choice| choice.1)).unwrap();
                let mut rng = thread_rng();
                for _ in 0..self.capacity {
                    let choice: &Enemies = &choices[dist.sample(&mut rng)].0;
                    match choice {
                        Enemies::Frog => self.frogs += 1,
                        Enemies::Lizard => self.lizards += 1,
                        Enemies::Mouse => self.mice += 1,
                        Enemies::Bird => self.birds += 1,
                    }
                    self.population = self.frogs + self.lizards + self.mice + self.birds;
                }
            }
            Biome::Prarie => {}
            Biome::Mountain => {}
        }
    }

    fn kill_enemy(&mut self, enemy: Enemies) {
        match enemy {
            Enemies::Frog => self.frogs -= 1,
            Enemies::Lizard => self.lizards -= 1,
            Enemies::Mouse => self.mice -= 1,
            Enemies::Bird => self.birds -= 1,
        }
        self.population = self.frogs + self.lizards + self.mice + self.birds;
    }
}

fn main() {
    let mut snake_player = Snake::new();
    let mut game_running = true;
    let mut area = Location::swamp();
    /* Swamp Area
    Frog Spawn Chance: 50%
    Lizard Spawn Chance: 35%
    Mouse Spawn Chance: 10%
    Bird Spawn Chance: 5% */

    while game_running {
        if area.population == 0 {
            println!("You are a snake.. find food to level up!");
            if snake_player.state != State::DiamondSnake {
                area.spawn_enemies();
                println!("Population: {}", area.population);
            } else if snake_player.state == State::DiamondSnake {
                println!("You've beat the game!");
                game_running = false;
            }
        } else {
            if area.frogs > 0 {
                let mut buffer = String::new();
                let stdin = io::stdin();
                println!("A frog is hopping! As a slithery snake it's your job to defend your territory! Kill the frog? Y/N");
                let _ = stdin.read_line(&mut buffer);

                match buffer.trim() {
                    "Y" => {
                        area.kill_enemy(Enemies::Frog);
                        println!("Frog KILLED!");

                        match snake_player.state {
                            State::Snake => {
                                println!("You feel something different, you notice you've grown bronze plated scales!");
                                snake_player.collect(Enemies::Frog);
                            }

                            _ => {
                                println!("\nPopulation: {}", area.population);
                                println!("Current Snake: {}", snake_player.state);
                                continue;
                            }
                        }
                    }

                    "N" => {
                        println!("Fool! The frog has attacked you and you have died!");
                        game_running = false;
                    }
                    _ => println!("Y/N"),
                }
            }
            if area.lizards > 0 {
                let mut buffer = String::new();
                let stdin = io::stdin();
                println!("A lizard is scurrying! As a slithery snake it's your job to defend your territory! Kill the lizard? Y/N");
                let _ = stdin.read_line(&mut buffer);

                match buffer.trim() {
                    "Y" => {
                        area.kill_enemy(Enemies::Lizard);
                        println!("Lizard KILLED!");
                        if snake_player.state != State::SilverSnake {
                            println!(
                                "You feel something different, your scales are now plated with silver!"
                            );
                            snake_player.collect(Enemies::Lizard);
                        }

                        println!("\n Population: {}", area.population);
                        println!("Current Snake: {}", snake_player.state);
                    }
                    "N" => {
                        println!(
                            "Fool! The Lizard has attacked you and you have lived, but barely..."
                        );
                        game_running = true;
                    }
                    _ => println!("Y/N"),
                }
            }
            if area.mice > 0 {
                let mut buffer = String::new();
                let stdin = io::stdin();
                println!("A mouse squeaks. As a slithery snake it's your job to defend your territory! Kill the mouse? Y/N");
                let _ = stdin.read_line(&mut buffer);

                match buffer.trim() {
                    "Y" => {
                        area.kill_enemy(Enemies::Mouse);
                        println!("Mouse KILLED!");
                        if snake_player.state != State::GoldSnake {
                            println!("You feel strange as gold begins to cover your scales! You feel sparkly and strong.");
                        }
                        snake_player.collect(Enemies::Mouse);
                        println!("\n Population: {}", area.population);
                        println!("Current Snake: {}", snake_player.state);
                    }
                    "N" => {
                        println!("Fool! The mouse has turned you into a block of cheese!");
                        game_running = false;
                    }
                    _ => println!("Y/N"),
                }
            }
            if area.birds > 0 {
                let mut buffer = String::new();
                let stdin = io::stdin();
                println!("A bird is flying! As a slithery snake it's your job to defend your territory! Kill the frog? Y/N");
                let _ = stdin.read_line(&mut buffer);

                match buffer.trim() {
                    "Y" => {
                        area.kill_enemy(Enemies::Bird);
                        println!("Bird KILLED!");
                        if snake_player.state != State::DiamondSnake {
                            println!("The chances were low.. but you've reached the final form! You are covered in diamonds, piercing fiercely through anything that tries to mess with you.");
                            snake_player.collect(Enemies::Bird);
                        }

                        println!("\n Population: {}", area.population);
                    }
                    "N" => {
                        println!("Fool! The bird clenches you in it's talons!");
                        game_running = false;
                    }
                    _ => println!("Y/N"),
                }
            }
        }
    }
}
