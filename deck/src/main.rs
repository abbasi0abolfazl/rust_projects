use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];
    
        let mut cards = Vec::new();
    
        for suit in suits{
            for value in values{
                let card = format!("{} of {}", suit, value);
                cards.push(card);
            }
        }

        Deck{ cards }
    }

    fn shuffle(&mut self){
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }
}


fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    // TODO: add Error handling for this method
    let cards = deck.deal(3);

    // two way for set empty vector in rust Vec::new() and vec![]
    // let deck = Deck{ cards: Vec::new() };
    // let deck = Deck{ cards: vec![] };

    println!("Heres your class: {:#?}", deck);
    println!("Heres your hand: {:#?}", cards);
}
