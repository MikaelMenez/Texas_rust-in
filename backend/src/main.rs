mod cards;
use cards::cards::{Card, DECK, Ranks, Suits};
fn main() {
    let card = Card::new(Ranks::Ace, Suits::Hearts);
    print!("{:?}\n", DECK)
}
