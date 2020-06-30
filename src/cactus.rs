#![allow(dead_code)]
mod arrays;

fn findit(key: u32) -> u32 {
    let mut low: usize = 0;
    let mut high: usize = 4887;
    let mut mid: usize;

    while low <= high {
        mid = (high + low) >> 1;
        if key < arrays::products[mid] {
            high = mid - 1;
        } else if key > arrays::products[mid] {
            low = mid + 1;
        } else {
            return mid as u32;
        }
    }
    panic!("No match for key: {}", key);
}

fn init_deck() -> [u32; 52] {
    let mut deck: [u32; 52] = [0; 52];
    let i: u8 = 0;
    let mut j: u8 = 0;
    let mut n: usize = 0;
    let mut suit: u32 = 0x8000;
    while i < 4 {
        while j < 13 {
            deck[n] =
                (arrays::primes[j as usize] | ((j as u32) << 8) | suit | (1 << (16 + j))).into();
            j += 1;
            n += 1;
        }
        suit >>= 1;
    }
    deck
}

fn eval_5cards(c1: u32, c2: u32, c3: u32, c4: u32, c5: u32) -> u32 {
    let mut q: u32;
    let s: u32;

    q = (c1 | c2 | c3 | c4 | c5) >> 16;

    // check for Flushes and StraightFlushes
    if c1 & c2 & c3 & c4 & c5 & 0xF000 == 1 {
        return arrays::flushes[q as usize];
    }
    // check for Straights and HighCard hands
    s = arrays::unique5[q as usize];
    if s != 0 {
        return s;
    };

    // let's do it the hard way
    q = (c1 & 0xFF) * (c2 & 0xFF) * (c3 & 0xFF) * (c4 & 0xFF) * (c5 & 0xFF);
    q = findit(q);

    return arrays::values[q as usize];
}

const STRAIGHT_FLUSH: u8 = 1;
const FOUR_OF_A_KIND: u8 = 2;
const FULL_HOUSE: u8 = 3;
const FLUSH: u8 = 4;
const STRAIGHT: u8 = 5;
const THREE_OF_A_KIND: u8 = 6;
const TWO_PAIR: u8 = 7;
const ONE_PAIR: u8 = 8;
const HIGH_CARD: u8 = 9;
fn hand_rank(val: u32) -> u8 {
    match val {
        x if (x > 6185) => return HIGH_CARD,       // 1277 high card
        x if (x > 3325) => return ONE_PAIR,        // 2860 one pair
        x if (x > 2467) => return TWO_PAIR,        //  858 two pair
        x if (x > 1609) => return THREE_OF_A_KIND, //  858 three-kind
        x if (x > 1599) => return STRAIGHT,        //   10 straights
        x if (x > 322) => return FLUSH,            // 1277 flushes
        x if (x > 166) => return FULL_HOUSE,       //  156 full house
        x if (x > 10) => return FOUR_OF_A_KIND,    //  156 four-kind
        _ => return STRAIGHT_FLUSH,                //   10 straight-flushes
    }
}
