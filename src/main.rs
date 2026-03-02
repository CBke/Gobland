// SUPER BASIC AI TEXT PARSER
// BY CRAFSMAN STEADY CRAFTIN

use std::collections::HashMap;
use std::io::{self, Write};
use rand::Rng;

struct GameState {
    location: i32,
    keep_jumping: i32,
    inventory: Vec<String>,
}

impl GameState {
    fn new() -> Self {
        Self {
            location: 1,
            keep_jumping: 0,
            inventory: Vec::new(),
        }
    }
}

// ---------- Commands ----------

fn cmd_jump(state: &mut GameState) {
    state.keep_jumping += 1;

    let responses = [
        "You jumped.",
        "You jumped. Very nice.",
        "You jumped, for no reason.",
    ];

    let mut rng = rand::thread_rng();
    let choice = rng.gen_range(0..responses.len());

    println!("{}", responses[choice]);

    if state.keep_jumping > 5 {
        println!("Please STOP jumping.");
    }
}

fn cmd_attack(_state: &mut GameState) {
    println!("You cannot attack without a weapon.");
}

fn cmd_inventory(state: &mut GameState) {
    println!("INVENTORY:");
    if state.inventory.is_empty() {
        println!("You are carrying nothing.");
    } else {
        for item in &state.inventory {
            println!("- {}", item);
        }
    }
}

fn cmd_north(state: &mut GameState) {
    if state.location < 4 {
        state.location += 1;
        println!("You move north. Location: {}", state.location);
    } else {
        println!("The path is blocked by a THORN THICKET.");
    }
}

fn cmd_south(state: &mut GameState) {
    if state.location > 1 {
        state.location -= 1;
        println!("You move south. Location: {}", state.location);
    } else {
        println!("You cannot go further south.");
    }
}

fn cmd_goblin(_state: &mut GameState) {
    println!("There is no goblin here.");
}
fn cmd_why(_state: &mut GameState) {
    let responses = [
        "Why would I know?",
        "I'm not sure why. Ask the developer.",
        "WHY are you asking?",
        "I'm not sure why.",
        "I don't know... why?",
    ];
    println!("{}", random_choice(&responses));
}

fn cmd_who(_state: &mut GameState) {
    let responses = [
        "I don't know who.",
        "How would I know who?",
        "I don't know who you are talking about.",
        "I can't talk about them.",
        "Whoever, I suppose.",
    ];
    println!("{}", random_choice(&responses));
}

fn cmd_where(_state: &mut GameState) {
    let responses = [
        "I don't know where.",
        "Wherever it is, I suppose.",
        "You have to search.",
        "Try looking.",
        "It is somewhere around here.",
    ];
    println!("{}", random_choice(&responses));
}

fn cmd_how(_state: &mut GameState) {
    let responses = [
        "How should I know?",
        "That is what you must learn.",
        "That's what this game is all about.",
        "However you think is best.",
        "However is most strategic.",
    ];
    println!("{}", random_choice(&responses));
}
fn cmd_look(state: &mut GameState) {
    match state.location {
        1 => {
            println!("You are standing at the entrance of a heavily wooded trail.");
            println!("A weathered wooden sign reads 'GOBLIN WOODS AHEAD'.");
        }
        2 => {
            println!("You are deeper in the forest. The trees are thick and the path narrows.");
        }
        3 => {
            println!("You reach a small clearing. Sunlight filters through the branches.");
        }
        4 => {
            println!("You are at the heart of the woods. It's eerily quiet here.");
        }
        _ => {
            println!("You look around, but there's nothing notable here.");
        }
    }
}

// helper functie
fn random_choice<'a>(choices: &'a [&'a str]) -> &'a str {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..choices.len());
    choices[idx]
}

// ---------- Dispatcher ----------

fn build_commands() -> HashMap<&'static str, fn(&mut GameState)> {
    let mut commands: HashMap<&str, fn(&mut GameState)> = HashMap::new();

    // Bewegings- en actiecommando's
    commands.insert("jump", cmd_jump);
    commands.insert("attack", cmd_attack);
    commands.insert("inventory", cmd_inventory);
    commands.insert("look", cmd_look);
    commands.insert("search", cmd_look);  // alias van look
    commands.insert("north", cmd_north);
    commands.insert("south", cmd_south);

    // Vraag-commando's
    commands.insert("why", cmd_why);
    commands.insert("who", cmd_who);
    commands.insert("where", cmd_where);
    commands.insert("how", cmd_how);

    // Combat / NPC
    commands.insert("goblin", cmd_goblin);
    commands
}

fn handle_input(input: &str, state: &mut GameState, commands: &HashMap<&str, fn(&mut GameState)>) {
    let input = input.to_lowercase();

    for (keyword, action) in commands {
        if input.contains(keyword) {
            action(state);
            return;
        }
    }

    println!("I don't understand.");
}

// ---------- Main ----------

fn main() {
    let mut state = GameState::new();
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
    println!("You are standing at the entrance of a heavily wooded trail.");
    println!("A weathered wooden sign reads 'GOBLIN WOODS AHEAD'.");
    println!();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        handle_input(input.trim(), &mut state, &commands);
    }
}
