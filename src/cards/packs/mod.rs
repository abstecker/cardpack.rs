mod standard52;

use crate::{ Pile, Rank, Suit};
use crate::cards::dealer::Shuffler;

pub trait Cardpack {

    /// Returns a sorted version of the specific deck in question.
    fn cardpack() -> Pile;
    fn suits() -> Vec<Suit>;
    fn ranks() -> Vec<Rank>;
    // fn shuffle(&mut self, T: dyn Shuffler<T>);



    // fn new() -> Self;
    // fn len(&self) -> usize;
    // fn is_empty(&self) -> bool;
    // fn shuffle(&mut self);
    // fn draw(&mut self) -> Option<Card>;
    // fn deal(&mut self, count: usize) -> Result<Vec<Card>, CardError>;
    // fn add(&mut self, card: Card) -> Result<(), CardError>;
    // fn add_all(&mut self, cards: Vec<Card>) -> Result<(), CardError>;
    // fn remove(&mut self, index: usize) -> Result<Card, CardError>;
    // fn remove_all(&mut self) -> Result<Vec<Card>, CardError>;
    // fn clear(&mut self);
    // fn contains(&self, card: Card) -> bool;
    // fn count(&self, card: Card) -> usize;
    // fn index_of(&self, card: Card) -> Option<usize>;
    // fn get(&self, index: usize) -> Option<Card>;
    // fn set(&mut self, index: usize, card: Card) -> Result<(), CardError>;
    // fn swap(&mut self, index1: usize, index2: usize) -> Result<(), CardError>;
    // fn sort(&mut self);
    // fn sort_by(&mut self, compare: fn(&Card, &Card) -> Ordering);
    // fn sort_by_key(&mut self, key: fn(&Card) -> u64);
    // fn sort_by_suit(&mut self);
    // fn sort_by_rank(&mut self);
    // fn sort_by_suit_rank(&mut self);
    // fn sort_by_rank_suit(&mut self);
    // fn sort_by_suit_rank_desc(&mut self);
    // fn sort_by_rank_suit_desc(&mut self);
    // fn sort_by_suit_rank_ace_low(&mut self);
    // fn sort_by_rank_suit_ace_low(&mut self);
    // fn sort_by_suit_rank_desc_ace_low(&mut self);
    // fn sort_by_rank_suit_desc_ace_low(&mut self);
    // fn sort_by_suit_rank_jack_low(&mut self);
    // fn sort_by_rank_suit_jack_low(&mut self);
    // fn sort_by_suit_rank_desc_jack_low(&mut self);
    // fn sort_by_rank_suit_desc_jack_low(&mut self);
    // fn sort_by_suit_rank_ace_low_desc(&mut self);
    // fn sort_by_rank_suit_ace_low_desc(&mut self);

}