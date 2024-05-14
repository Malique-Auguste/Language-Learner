mod deck;

use deck::*;
use serde::de;

use std::{fs, io::{Read, Write}};


fn main() {
    let mut database_file = fs::OpenOptions::new().read(true).write(true).create(true).open("database.txt").unwrap();
    let mut database_content = String::new();
    database_file.read_to_string(&mut database_content).unwrap();

    let mut deck:Deck = {
        if database_content == String::from("") {
            Deck::default()
        }
        else {
            serde_json::from_str(&database_content).unwrap()
        }
    };


    loop {
        println!("\n----------\n");
        println!("Would you like to test yourself (1) or add to the deck (2)? (Type 'x' at any time to exit)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == String::from("x") {
            break
        }

        let option_num: u8 = input.trim().parse().unwrap();

        match option_num {
            1 => {
                println!("\n----------\n");
                if deck.inner().len() == 0 {
                    println!("Cannot operate on an empty deck");
                }
                else {
                    loop {
                        match test_deck(&mut deck) {
                            Ok(_) => (),
                            Err(_) => break
                        }
                    }
                }   
            },
            2 => {
                loop {
                    println!("\n----------\n");
                    let card = match create_card() {
                        Ok(c) => c,
                        Err(_) => break
                    };
                    
                    deck.extend(card)
                }
            },
            _ => {
                println!("Unimplemented");
            }
        }
    }

    let mut database_file = fs::OpenOptions::new().write(true).open("database.txt").unwrap();
    database_file.write_all(serde_json::to_string_pretty(&deck).unwrap().as_bytes()).unwrap();

}

fn test_deck(deck: &mut Deck) -> Result<(), ()> {
    for card in deck.inner().iter_mut() {
        println!("Card Front: {}", card.front());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == String::from("x") {
            return Err(())
        }

        println!("Card Back: {}\nDid you get it correct (1), or wrong (2)?", card.back());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == String::from("x") {
            return Err(())
        }

        let option_num: u8 = input.trim().parse().unwrap();

        match option_num {
            1 => card.adjust_accuracy(true),
            2 => card.adjust_accuracy(false),
            _ => println!("Unimplemented")

        }
    }

    Ok(())
}


fn create_card() -> Result<Card, ()> {
    let mut card_front = String::new();
    println!("Card Front: ");
    std::io::stdin().read_line(&mut card_front).unwrap();

    if card_front.trim().to_lowercase() == String::from("x") {
        return Err(())
    }
    
    let mut card_back = String::new();
    println!("Card Back: ");
    std::io::stdin().read_line(&mut card_back).unwrap();

    if card_back.trim().to_lowercase() == String::from("x") {
        return Err(())
    }

    Ok(Card::new(card_front, card_back))
}
