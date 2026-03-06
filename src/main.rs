// SUPER BASIC AI TEXT PARSER
// BY CRAFSMAN STEADY CRAFTIN

use std::collections::HashMap;
use std::io::{self, Write};
use rand::Rng;

// ---------------- Context ----------------

#[derive(Debug)]
enum Location {
    TrailEntrance,
    ForestPath,
    Clearing,
    DeepWoods,
}

struct GameState {
    location: Location,
    keep_jumping: i32,
    inventory: Vec<String>,
}

impl GameState {
    fn new() -> Self {
        Self {
            location: Location::TrailEntrance,
            keep_jumping: 0,
            inventory: Vec::new(),
        }
    }
}

struct GameContext {
    state: GameState,
    rng: rand::rngs::ThreadRng,
}

impl GameContext {
    fn new() -> Self {
        Self {
            state: GameState::new(),
            rng: rand::thread_rng(),
        }
    }
}

// ---------------- Macros ----------------

trait RandomChoice<T> {
    fn random_choice(&self, rng: &mut rand::rngs::ThreadRng) -> &T;
}

impl<T> RandomChoice<T> for [T] {
    fn random_choice(&self, rng: &mut rand::rngs::ThreadRng) -> &T {
        &self[rng.gen_range(0..self.len())]
    }
}


macro_rules! command {
    ($param:ident => $body:expr) => {
        Box::new(|$param: &mut GameContext| $body) as Box<dyn Fn(&mut GameContext)>
    };
}
// ---------------- Command Table ----------------
type Command = Box<dyn Fn(&mut GameContext)>;

fn build_commands() -> HashMap<&'static str, Command> {
    let mut commands: HashMap<&str, Command> = HashMap::new();

    commands.insert("jump", command!(x => {
        x.state.keep_jumping += 1;
        let responses = [
            "You jumped.",
            "You jumped. Very nice.",
            "You jumped for no reason."
        ];
        println!("{}", responses.random_choice(&mut x.rng));
        if x.state.keep_jumping > 5 {
            println!("Please STOP jumping.");
        }
    }));

    commands.insert("attack", command!(_x => {
        println!("You cannot attack without a weapon.");
    }));

    commands.insert("inventory", command!(x => {
        println!("INVENTORY:");
        if x.state.inventory.is_empty() {
            println!("You are carrying nothing.");
        } else {
            for item in &x.state.inventory {
                println!("- {}", item);
            }
        }
    }));

    commands.insert("look", command!(x => {
        match x.state.location {
            Location::TrailEntrance => {
                println!("You are at the entrance of a heavily wooded trail.");
                println!("A weathered wooden sign reads 'GOBLIN WOODS AHEAD'.");
            }
            Location::ForestPath => println!("You are deeper in the forest. The trees are thick."),
            Location::Clearing => println!("You reach a small clearing. Sunlight filters through."),
            Location::DeepWoods => println!("You are at the heart of the woods. It's eerily quiet."),
        }
    }));

    commands.insert("search", command!(x => {
        match x.state.location {
            Location::TrailEntrance => println!("You examine the entrance carefully."),
            Location::ForestPath => println!("You search the forest path."),
            Location::Clearing => println!("You look around the clearing."),
            Location::DeepWoods => println!("You explore the deep woods."),
        }
    }));

    // Movement commands (no capturing of `commands`)
    commands.insert("north", command!(x => {
        x.state.location = match x.state.location {
            Location::TrailEntrance => { println!("You move north."); Location::ForestPath },
            Location::ForestPath => { println!("You move north."); Location::Clearing },
            Location::Clearing => { println!("You move north."); Location::DeepWoods },
            Location::DeepWoods => { println!("The path is blocked by a THORN THICKET."); Location::DeepWoods },
        };
    }));

    commands.insert("south", command!(x => {
        x.state.location = match x.state.location {
            Location::DeepWoods => { println!("You move south."); Location::Clearing },
            Location::Clearing => { println!("You move south."); Location::ForestPath },
            Location::ForestPath => { println!("You move south."); Location::TrailEntrance },
            Location::TrailEntrance => { println!("You cannot go further south."); Location::TrailEntrance },
        };
    }));

    commands.insert("goblin", command!(_x => {
        println!("There is no goblin here.");
    }));

    commands.insert("why", command!(x => {
        let responses = [
            "Why would I know?",
            "Ask the developer.",
            "WHY are you asking?",
            "I'm not sure why.",
            "I don't know... why?"
        ];
        println!("{}", responses.random_choice(&mut x.rng));
    }));

    commands.insert("who", command!(x => {
        let responses = [
            "I don't know who.",
            "How would I know who?",
            "I can't talk about them.",
            "Whoever, I suppose."
        ];
        println!("{}", responses.random_choice(&mut x.rng));
    }));

    commands
}


fn handle_input(input: &str, game_context: &mut GameContext, commands: &HashMap<&str, Box<dyn Fn(&mut GameContext)>>) {

    let input = input.to_lowercase();

    for (keyword, action) in commands {
        if input.contains(keyword) {
            action(game_context);
            return;
        }
    }

    println!("I don't understand.");
}
//---------------- Main Loop ----------------

fn main() {

    let mut ctx = GameContext::new();
    let commands = build_commands();

    println!("████████████████████████████████████████████████████████████████████████████████");
    println!("███      ██████████████████████████████████████████████ ████████████████████████");
    println!("███ ████ ██████████ ████████████████    ███████████████ ████████████████████████");
    println!("███ ███████      ██ ██████ ████████████ ███████████████ ███████░░██████░░░░░████");
    println!("███ ███  ██ ████ ██    ███ ███████    █ ████    ███     ████████░██████░███░████");
    println!("███ ████ ██ ████ ██ ██ ███ ███████ ████ ████ ██ ███ ███ ████████░██████░███░████");
    println!("███      ██      ██    ███     ███      ████ ██ ███     ████████░██░░██░░░░░████");
    println!("████████████████████████████████████████████████████████████████████████████████");
    println!("██                                                                            ██");
    println!("██                     Interactive Text Adventure System                      ██");
    println!("██                                                                            ██");
    println!("████████████████████████████████████████████████████████████████████████████████");
    
    println!();

    // start location description
    if let Some(start_cmd) = commands.get("look") {
        start_cmd(&mut ctx);
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        handle_input(input.trim(), &mut ctx, &commands);
    }
}
