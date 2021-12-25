use crate::cards::card_error::CardError;
use crate::{ACE, Card, EIGHT, FIVE, FOUR, JACK, KING, NINE, QUEEN, Rank, SEVEN, SIX, Standard52, TEN, THREE, TWO};
use bitvec::field::BitField;
use bitvec::prelude::{BitArray, BitSlice, Msb0};
use std::fmt::{Display, Formatter};
use wyz::FmtForward;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitCard(BitArray<Msb0, [u8; 4]>);

pub struct FiveBitCards([BitCard; 5]);

impl BitCard {
    // Constructors
    #[must_use]
    pub fn new(b: BitArray<Msb0, [u8; 4]>) -> BitCard {
        BitCard(b)
    }

    #[must_use]
    #[allow(clippy::needless_borrow)]
    pub fn new_from_card(card: &Card) -> BitCard {
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_rank(&card);
        bit_card.set_rank_flag(&card);
        bit_card.set_rank_prime(&card);
        bit_card.set_suit(&card);
        bit_card
    }

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    pub fn new_from_index(i: &'static str) -> Result<BitCard, CardError> {
        let c = Standard52::card_from_index(i);

        if c.is_valid() {
            Ok(BitCard::new_from_card(&c))
        } else {
            Err(CardError::InvalidCard)
        }
    }

    #[must_use]
    pub fn new_from_u64(integer: u64) -> BitCard {
        let mut bc: BitCard = BitCard::default();
        bc.0[..32].store_be(integer);
        bc
    }

    // Struct methods

    /// Takes the `BitArray` representation of the Card and returns a `String`
    /// representation of the bits. If split is set to true, it will put a space
    /// between each bite. For instance, `00001000000000000100101100100101`
    /// becomes `00001000 00000000 01001011 00100101`.
    #[must_use]
    pub fn display(&self, split: bool) -> String {
        let mut word_string = String::with_capacity(35);
        let start_bit: usize = 0;
        let bits = start_bit..start_bit + 32;
        for (bit, idx) in self.0.as_bitslice().iter().by_val().zip(bits) {
            word_string.push_str(if bit { "1" } else { "0" });
            if split && idx % 8 == 7 && idx % 32 != 31 {
                word_string.push(' ');
            }
        }
        word_string
    }

    #[must_use]
    pub fn get_rank(&self) -> Rank {
        match format!("{:b}", self.get_rank_bits_slice()).as_str() {
            "[00010000, 00000000]" => Rank::new(ACE),
            "[00001000, 00000000]" => Rank::new(KING),
            "[00000100, 00000000]" => Rank::new(QUEEN),
            "[00000010, 00000000]" => Rank::new(JACK),
            "[00000001, 00000000]" => Rank::new(TEN),
            "[00000000, 10000000]" => Rank::new(NINE),
            "[00000000, 01000000]" => Rank::new(EIGHT),
            "[00000000, 00100000]" => Rank::new(SEVEN),
            "[00000000, 00010000]" => Rank::new(SIX),
            "[00000000, 00001000]" => Rank::new(FIVE),
            "[00000000, 00000100]" => Rank::new(FOUR),
            "[00000000, 00000010]" => Rank::new(THREE),
            "[00000000, 00000001]" => Rank::new(TWO),
            _ => Rank::default()
        }
    }

    #[must_use]
    pub fn get_rank_bits_slice(&self) -> &BitSlice<Msb0, u8> {
        &self.0[..16]
    }

    /// Returns a `BitSlice` of the `Suit` section of the `CactusKev` `BitArray`.
    #[must_use]
    pub fn get_suit_slice(&self) -> &BitSlice<Msb0, u8> {
        &self.0[16..20]
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.0.count_zeros() == 32
    }

    // Private methods

    fn set_rank(&mut self, card: &Card) {
        self.0[20..24].store_be(card.rank.weight);
    }

    fn set_rank_flag(&mut self, card: &Card) {
        match card.rank.weight {
            12 => self.0.set(3, true), // Ace
            11 => self.0.set(4, true), // King
            10 => self.0.set(5, true), // Queen
            9 => self.0.set(6, true),  // Jack
            8 => self.0.set(7, true),  // Ten
            7 => self.0.set(8, true),  // Nine
            6 => self.0.set(9, true),  // Eight
            5 => self.0.set(10, true), // Seven
            4 => self.0.set(11, true), // Six
            3 => self.0.set(12, true), // Five
            2 => self.0.set(13, true), // Four
            1 => self.0.set(14, true), // Three
            0 => self.0.set(15, true), // Two
            _ => (),
        }
    }

    fn set_rank_prime(&mut self, card: &Card) {
        self.0[26..32].store_be(card.rank.prime);
    }

    fn set_suit(&mut self, card: &Card) {
        match card.suit.weight {
            4 => self.0.set(19, true), // Spades
            3 => self.0.set(18, true), // Hearts
            2 => self.0.set(17, true), // Diamonds
            1 => self.0.set(16, true), // Clubs
            _ => (),
        }
    }
}

impl Default for BitCard {
    fn default() -> BitCard {
        BitCard::new(BitArray::zeroed())
    }
}

/// [Module ``std::fmt``](https://doc.rust-lang.org/std/fmt/)
/// ```txt
/// +--------+--------+--------+--------+
/// |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
/// +--------+--------+--------+--------+
///
/// p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
/// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
/// cdhs = suit of card (bit turned on based on suit of card)
/// b = bit turned on depending on rank of card
/// ```
impl Display for BitCard {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = fmt.debug_list();

        let mut mark_string = String::with_capacity(35);
        mark_string.push_str("xxxAKQJT 98765432 CDHSrrrr xxpppppp");

        out.entry(&(self.display(true)).fmt_display());
        out.entry(&(&mark_string).fmt_display());
        out.finish()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod bit_card_tests {
    use super::*;
    use crate::{Standard52};

    #[test]
    fn len() {
        assert_eq!(BitCard::default().0.len(), 32);
    }

    #[test]
    fn new_from_card() {
        let card = Standard52::card_from_index("K♦");
        let cactusKevCard: BitCard = BitCard::new_from_card(&card);

        assert_eq!(
            "00001000 00000000 01001011 00100101",
            cactusKevCard.display(true)
        );
    }

    /// This test goes through all 52 cards in a Standard52 deck and compares the
    /// `CactusKevCard` version of the bite signature with the `Card`'s version.
    #[test]
    fn new_from_card__complete() {
        let standard52 = Standard52::default();
        for card in standard52.deck {
            let cactusKevCard: BitCard = BitCard::new_from_card(&card);
            let s = format!("{:032b}", card).to_string();
            assert_eq!(s, cactusKevCard.display(false));
        }
    }

    #[test]
    fn new_from_index() {
        let card = Standard52::card_from_index("KS");
        let expected = BitCard::new_from_card(&card);

        let actual = BitCard::new_from_index("KS").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn new_from_index__invalid() {
        assert!(BitCard::new_from_index("xx").is_err());
    }

    #[test]
    fn new_from_u64() {
        let ace_spades: u64 = 268442665;
        let s = "00010000 00000000 00011100 00101001".to_string();
        let actual = BitCard::new_from_u64(ace_spades);

        assert_eq!(actual.display(true), s);
        assert_eq!(actual, BitCard::new_from_index("A♤").unwrap());
    }

    #[test]
    fn get_rank() {
        assert_eq!(BitCard::new_from_index("AS").unwrap().get_rank(), Rank::new(ACE));
        assert_eq!(BitCard::new_from_index("KS").unwrap().get_rank(), Rank::new(KING));
        assert_eq!(BitCard::new_from_index("QS").unwrap().get_rank(), Rank::new(QUEEN));
        assert_eq!(BitCard::new_from_index("JS").unwrap().get_rank(), Rank::new(JACK));
        assert_eq!(BitCard::new_from_index("TS").unwrap().get_rank(), Rank::new(TEN));
        assert_eq!(BitCard::new_from_index("9S").unwrap().get_rank(), Rank::new(NINE));
        assert_eq!(BitCard::new_from_index("8S").unwrap().get_rank(), Rank::new(EIGHT));
        assert_eq!(BitCard::new_from_index("7S").unwrap().get_rank(), Rank::new(SEVEN));
        assert_eq!(BitCard::new_from_index("6S").unwrap().get_rank(), Rank::new(SIX));
        assert_eq!(BitCard::new_from_index("5S").unwrap().get_rank(), Rank::new(FIVE));
        assert_eq!(BitCard::new_from_index("4S").unwrap().get_rank(), Rank::new(FOUR));
        assert_eq!(BitCard::new_from_index("3S").unwrap().get_rank(), Rank::new(THREE));
        assert_eq!(BitCard::new_from_index("2S").unwrap().get_rank(), Rank::new(TWO));
    }

    #[test]
    fn get_rank_bits() {
        let card: BitCard = BitCard::new_from_index("KS").unwrap();
        assert_eq!(
            "[00001000, 00000000]",
            format!("{:b}", card.get_rank_bits_slice())
        );
    }

    #[test]
    fn get_suit_slice() {
        let card: BitCard = BitCard::new_from_index("KS").unwrap();
        assert_eq!("[0001]", format!("{:04b}", card.get_suit_slice()));

        let card: BitCard = BitCard::new_from_index("KH").unwrap();
        assert_eq!("[0010]", format!("{:04b}", card.get_suit_slice()));

        let card: BitCard = BitCard::new_from_index("K♦").unwrap();
        assert_eq!("[0100]", format!("{:04b}", card.get_suit_slice()));

        let card: BitCard = BitCard::new_from_index("KC").unwrap();
        assert_eq!("[1000]", format!("{:04b}", card.get_suit_slice()));
    }

    #[test]
    fn is_blank() {
        assert!(BitCard::default().is_blank());
        assert!(!BitCard::new_from_index("KS").unwrap().is_blank());
    }

    #[test]
    fn is_blank__false() {
        assert!(!BitCard::new_from_index("KS").unwrap().is_blank());
    }

    #[test]
    fn set_rank() {
        let mut bit_card: BitCard = BitCard::default();
        let card = Standard52::card_from_index("K♦");

        bit_card.set_rank(&card);
        assert_eq!(
            "00000000 00000000 00001011 00000000",
            bit_card.display(true)
        );
    }

    #[test]
    fn set_rank_flag() {
        let mut bit_card: BitCard = BitCard::default();
        let card = Standard52::card_from_index("K♦");

        bit_card.set_rank_flag(&card);
        assert_eq!(
            "00001000 00000000 00000000 00000000",
            bit_card.display(true)
        );
    }

    #[test]
    fn set_rank_prime() {
        let mut bit_card: BitCard = BitCard::default();
        let card = Standard52::card_from_index("K♦");

        bit_card.set_rank_prime(&card);
        assert_eq!(
            "00000000 00000000 00000000 00100101",
            bit_card.display(true)
        );
    }

    #[test]
    fn set_suit() {
        let mut bit_card: BitCard = BitCard::default();

        let card = Standard52::card_from_index("KS");
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 00010000 00000000",
            bit_card.display(true)
        );

        let card = Standard52::card_from_index("KH");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 00100000 00000000",
            bit_card.display(true)
        );

        let card = Standard52::card_from_index("K♦");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 01000000 00000000",
            bit_card.display(true)
        );

        let card = Standard52::card_from_index("KC");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 10000000 00000000",
            bit_card.display(true)
        );
    }
}
