use serde::{Deserialize, Serialize};

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
                unreachable!()
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
                unreachable!()
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