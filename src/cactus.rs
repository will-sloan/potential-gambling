#![allow(dead_code)]
mod arrays;


#![allow(dead_code)]
mod arrays;
//extern crate libc;

// fn main() {
//     let a = init_deck();
//     for i in a.iter() {
//         println!("{}", i);
//     }
// }
// #[link(name = "pokerlib")]
// extern {
//     fn init_deck
// }

// extern "C" {
//     fn abs(input: i32) -> i32;
// }
fn main() {
    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    let value_str = vec![
        "",
        "Straight Flush",
        "Four of a Kind",
        "Full House",
        "Flush",
        "Straight",
        "Three of a Kind",
        "Two Pair",
        "One Pair",
        "High Card",
    ];
    //let (deck, hand, freq): ([u32;52], [u32,5], [u32;10]) = ([0;52], [0;5], [0;10]);
    let mut deck: [u32; 52] = [0; 52];
    let mut hand: [u32; 5] = [0; 5];
    let mut freq: [u32; 10] = [0; 10];
    //let (mut a, mut b, mut c, mut d, mut e, mut i, mut j): (u32, u32, u32, u32, u32, u32, u32) = (0, 0, 0, 0, 0, 0, 0);
    //int a, b, c, d, e, i, j;

    // Seed the random number generator.
    //srand48(getpid());

    // Initialize the deck.
    let deck = init_deck();

    // for (i, val) in deck.iter().enumerate() {
    //     println!("{} {}", i, val);
    // }
    //println!("1");
    // Zero out the frequency array.
    for i in 0..freq.len() {
        freq[i] = 0;
    }
    //println!("2");
    // for (i = 0; i < 10; i++)
    // 	freq[i] = 0;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    //println!("3");
    // for a in 0..48 {
    //     hand[0] = deck[a];
    //     let mut b = a + 1;
    //     while b < 49 {
    //         let mut c = b + 1;
    //         while c < 50 {
    //             let mut d = c + 1;
    //             while d < 51 {
    //                 let mut e = d + 1;
    //                 while e < 52 {
    //                     hand[4] = deck[e];
    //                     i = eval_5cards(hand[0], hand[0], hand[0], hand[0], hand[0]);
    //                     j = hand_rank(i);
    //                     freq[j as usize] += 1;
    //                     e += 1;
    //                 }
    //                 d += 1;
    //             }
    //             c += 1;
    //         }
    //         b += 1;
    //     }
    // }
    for i1 in 0..52 {
        //println!("5 part: {}", i1);
        for i2 in (i1 + 1)..52 {
            //println!("i2: {}", i2);
            for i3 in (i2 + 1)..52 {
                //println!("i3: {}", i3);
                for i4 in (i3 + 1)..52 {
                    //println!("i4: {}", i4);
                    for i5 in (i4 + 1)..52 {
                        //println!("i1: {} i2: {} i3: {} i4 {} i5: {}", i1, i2, i3, i4, i5);
                        hand[0] = deck[i1];
                        hand[1] = deck[i2];
                        hand[2] = deck[i3];
                        hand[3] = deck[i4];
                        hand[4] = deck[i5];
                        //println!("hand is: {:?}", hand);
                        let i = eval_5cards(hand[0], hand[1], hand[2], hand[3], hand[4]);
                        //println!("I is after eval_5: {}", i);
                        let j = hand_rank(i);
                        //println!("j is after hand_rank: {}", j);
                        freq[j as usize] += 1;

                        // mark the rank in the map
                        //rank_count.entry(rank).or_insert(true);
                    }
                }
            }
        }
    }
    //println!("6");
    for q in 1..10 {
        println!("{}: {}", value_str[q as usize], freq[q as usize])
    }
    // Loop over every possible five-card hand.
    // for (a = 0; a < 48; a++)
    // {
    // 	hand[0] = deck[a];
    // 	for (b = a + 1; b < 49; b++)
    // 	{
    // 		hand[1] = deck[b];
    // 		for (c = b + 1; c < 50; c++)
    // 		{
    // 			hand[2] = deck[c];
    // 			for (d = c + 1; d < 51; d++)
    // 			{
    // 				hand[3] = deck[d];
    // 				for (e = d + 1; e < 52; e++)
    // 				{
    // 					hand[4] = deck[e];

    // 					i = eval_5hand(hand);
    // 					j = hand_rank(i);
    // 					freq[j]++;
    // 				}
    // 			}
    // 		}
    // 	}
    // }

    // for (i = 1; i <= 9; i++)
    // 	printf("%15s: %8d\n", value_str[i], freq[i]);
}

fn findit(key: u32) -> u32 {
    // let mut low: usize = 0;
    // let mut high: usize = 4887;
    // let mut mid: usize;
    let spot = arrays::products.binary_search(&key).unwrap();
    spot as u32
    // match spot {
    //     Ok(x) => x,
    //     Err(why) => {
    //         Err(why)
    //         //panic!("Couldnt find key in binary serach: {}", why);
    //         //0
    //     }
    // }
    // while low <= high {
    //     mid = (high + low) >> 1;
    //     if key < arrays::products[mid] {
    //         println!(
    //             "key: {key} arrays::products[mid]: {arr} High: {high} Mid: {mid}",
    //             high = high,
    //             mid = mid,
    //             key = key,
    //             arr = arrays::products[mid]
    //         );
    //         high = mid - 1;
    //     } else if key > arrays::products[mid] {
    //         println!(
    //             "key: {key} arrays::products[mid]: {arr} Low: {low} Mid: {mid}",
    //             low = low,
    //             mid = mid,
    //             key = key,
    //             arr = arrays::products[mid]
    //         );
    //         low = mid + 1;
    //     } else {
    //         return mid as u32;
    //     }
    // }
    //panic!("No match for key: {}", key);
}

fn init_deck() -> [u32; 52] {
    let mut deck: [u32; 52] = [0; 52];
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut n: u32 = 0;
    let mut suit: u32 = 0x8000;
    while i < 4 {
        //println!();
        //println!("I is: {}", i);
        while j < 13 {
            //println!("J is: {}", j);
            deck[n as usize] =
                (arrays::primes[j as usize] | ((j as u32) << 8) | suit | (1 << (16 + j))).into();
            j += 1;
            n += 1;
        }
        j = 0;
        i += 1;
        suit >>= 1;
    }
    //println!("Done in init_deck");
    deck
}

fn eval_5cards(c1: u32, c2: u32, c3: u32, c4: u32, c5: u32) -> u32 {
    let mut q: u32;
    let s: u32;

    q = (c1 | c2 | c3 | c4 | c5) >> 16;

    // check for Flushes and StraightFlushes
    if c1 & c2 & c3 & c4 & c5 & 0xF000 != 0 {
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

const STRAIGHT_FLUSH: u32 = 1;
const FOUR_OF_A_KIND: u32 = 2;
const FULL_HOUSE: u32 = 3;
const FLUSH: u32 = 4;
const STRAIGHT: u32 = 5;
const THREE_OF_A_KIND: u32 = 6;
const TWO_PAIR: u32 = 7;
const ONE_PAIR: u32 = 8;
const HIGH_CARD: u32 = 9;
fn hand_rank(val: u32) -> u32 {
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


/*
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
*/
