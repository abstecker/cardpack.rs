use crate::Pile;

pub trait Shuffler<T> {
    fn shuffle(pile: &Pile);
}

