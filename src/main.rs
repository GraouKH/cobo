use rand::thread_rng;
use rand::seq::SliceRandom;
use std::{thread, time};
use std::io::{Write, stdout, Stdout, Read, BufRead};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

#[allow(dead_code)]
struct Player {
    id: u8,
    score: u8,
    cards: Vec<i8>
}

fn display(players: Vec<Player>) {
    for player in players {
        print!("{}: ", player.id);
        for card in player.cards {
            print!("{} ", card);
        }
        println!("");
    }
}

fn write_text(txt: &str) {
    let mut st = stdout();
    for c in txt.as_bytes().iter() {
        st.write(&[*c]).unwrap();
        thread::sleep(time::Duration::from_millis(50));
        st.flush().unwrap();
    }
}

fn introduction() {
    write_text("Hello,\nWelcome to CABO.\n");
    stdout().write(b"--------------\n").unwrap();
    write_text(" 1 - Play a game\n");
    write_text(" 2 - Rules\n");
    println!("{}", choice_until_valid_range(2));
}

fn choice_until_valid_range(range: u8) -> u8 {
    let c = choice_until_valid_int(range);
    return match c {
        e if e <= range => e,
        _ => {
            println!("Choose a number in range");
            choice_until_valid_range(range)
        }
    }
}

fn choice_until_valid_int(range: u8) -> u8 {
    let mut c = String::new();
    std::io::stdin().read_line(&mut c).unwrap();
    return match c.trim_end().parse::<u8>() {
        Ok(i) => i,
        Err(_) => {
            println!("Choose a valid number!\n");
            choice_until_valid_int(range)
        },
    }; 
}

fn rules() {
    println!("rules");
    introduction();
}

fn main() {
    // CLI cabo solo player game
    println!("CABO the card game.");

    // Deck initialisation
    let mut deck: Vec<i8> = (-1..=50).map(|x| x % 13 + 1).collect();
    deck.shuffle(&mut thread_rng());
    
    // Players initialisation
    let mut players: Vec<Player> = Vec::new();
    for i in 1..=4 {
        players.push(Player { id: (i), score: (0), cards: deck.split_off(deck.len() - 4) });
    }

    print!("{}[2J", 27 as char);
    // display(players);

    // TODO initilise starting state
    // TODO main loop
    // TODO add help rules

    introduction();
}
