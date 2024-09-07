use crate::cards::packs::Cardpack;
use crate::{Pile, Rank, Suit, ACE, CLUBS, DIAMONDS, EIGHT, FIVE, FOUR, HEARTS, JACK, KING, NINE, QUEEN, SEVEN, SIX, SPADES, TEN, THREE, TWO};

pub struct Standard52 {
    cards: Pile,
    pack: Pile,
}

impl Standard52 {

}

impl Default for Standard52 {
    fn default() -> Self {
        Standard52 {
            cards: Self::cardpack(),
            pack: Self::cardpack(),
        }
    }
}

impl Cardpack for Standard52 {
    fn cardpack() -> Pile {
        let mut cards: Pile = Pile::default();
        cards.fold_in(&Self::suits(), &Self::ranks());
        cards
    }

    fn suits() -> Vec<Suit> {
        Suit::from_array(&[SPADES, HEARTS, DIAMONDS, CLUBS])
    }

    fn ranks() -> Vec<Rank> {
        Rank::from_array(&[
            ACE, KING, QUEEN, JACK, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO,
        ])
    }
}