use core::num;

use serde::{Deserialize, Serialize};
use rand::prelude::*;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Deck {
    inner: Vec<Card>
}

impl Deck {
    pub fn extend(&mut self, card: Card) {
        self.inner.push(card)
    }

    pub fn inner(&mut self) -> &mut Vec<Card> {
        &mut self.inner
    }

    pub fn sort(&mut self){
        self.inner.sort_by(|a, b| b.accuracy[0].total_cmp(&a.accuracy[0]));
    }


    pub fn get_test_indices<'a>(&mut self, mut num_of_cards: usize, difficulty: Difficulty) -> Vec<usize> {
        self.sort();
        
        let mut indices: Vec<usize> = (0..self.inner.len()).collect();
        let mut output: Vec<usize> = Vec::new();
        
        let mut rng = rand::thread_rng();

        if num_of_cards > self.inner.len() {
            num_of_cards = self.inner.len()
        }

        for _ in 0..num_of_cards {
            let mut i = rng.gen_range(0..indices.len());
            
            match difficulty {
                Difficulty::Easy => i = i/2,
                Difficulty::Medium => (),
                Difficulty::Hard => i = i*2
            }

            if i > indices.len() {
                i = indices.len()
            }

            output.push(
                indices.remove(i)
            );
        }

        output
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Card {
    front: String,
    back: String,
    accuracy: [f64; 2]
}

impl Card {
    pub fn new(front: String, back: String) -> Card {
        Card {
            front: front.trim().to_lowercase(), 
            back: back.trim().to_lowercase(),
            accuracy: [50.0, 40.0]
        }
    }

    pub fn front(&self) -> &String {
        &self.front
    }

    pub fn back(&self) -> &String {
        &self.back
    }

    pub fn get_accuracy(&self) -> f64 {
        self.accuracy[0]
    }

    pub fn adjust_accuracy(&mut self, correct: bool) {
        let difference = self.accuracy[0] - self.accuracy[1];
        self.accuracy[1] = self.accuracy[0];

        if correct {
            if difference > 0.0 {
                //If you got it correct and previously got it correct increase by a lot
                self.accuracy[0] += difference * 1.5;
            }
            else if difference < 0.0 {
                //If you got it correct and previously got it wrong increase by a little
                self.accuracy[0] += 10.0;
            }
            else {
                if self.accuracy[0] == 100.0 {
                    self.accuracy[1] = 90.0
                }
                else {
                    unreachable!()
                }
            }
        }
        else {
            if difference > 0.0 {
                //If you got it wrong and previously got it correct decrease by a little
                self.accuracy[0] -= 10.0;
            }
            else if difference < 0.0 {
                //If you got it wrong and previously got it wrong decrease by a lot
                self.accuracy[0] -= -difference * 1.5;
            }
            else {
                if self.accuracy[0] == 0.0 {
                    self.accuracy[1] = 10.0
                }
                else {
                    unreachable!()
                }
            }
        }

        if self.accuracy[0] > 100.0 {
            self.accuracy[0] = 100.0
        }
        else if self.accuracy[0] < 0.0 {
            self.accuracy[0] = 0.0
        }
    }
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard
}