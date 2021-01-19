use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;
use std::time::Instant;
#[derive(Debug, Default, Clone)]
struct Cnt {
    ones: u32,
    twos: u32,
    threes: u32,
    fours: u32,
    fives: u32,
    sixs: u32,
}
enum Dices {
    ones,
    twos,
    threes,
    fours,
    fives,
    sixs,
    errors,
}

fn main() {
    // Roll some D6 dices
    let mut rng = rand::thread_rng();
    let dices: Vec<u8> = (0..20_000_000)
        .map(|_| Uniform::from(1..=6).sample(&mut rng))
        .collect();

    // imperative dice count
    let time = Instant::now();
    dbg!(imp_dice_count(&dices));
    println!("imp_dice_count {:.1?}", time.elapsed());

    // functional dice count
    let time = Instant::now();
    dbg!(fun_dice_count(&dices));
    println!("fun_dice_count {:.1?}", time.elapsed());

    // TODO Parallell functional dice count
    let time = Instant::now();
    dbg!(par_fun_dice_count(&dices));
    println!("par_fun_dice_count {:.1?}", time.elapsed());

}

fn imp_dice_count(dices: &[u8]) -> Cnt {
    let mut cnt = Cnt::default();
    for dice in dices {
        match dice {
            1 => cnt.ones += 1,
            2 => cnt.twos += 1,
            3 => cnt.threes += 1,
            4 => cnt.fours += 1,
            5 => cnt.fives += 1,
            6 => cnt.sixs += 1,
            _ => (),
        }
    }
    cnt
}

fn fun_dice_count(dices: &[u8]) -> Cnt {
    dices.iter().fold(Cnt::default(), |cnt, n| match n {
        1 => Cnt {
            ones: cnt.ones + 1,
            ..cnt
        },
        2 => Cnt {
            twos: cnt.twos + 1,
            ..cnt
        },
        3 => Cnt {
            threes: cnt.threes + 1,
            ..cnt
        },
        4 => Cnt {
            fours: cnt.fours + 1,
            ..cnt
        },
        5 => Cnt {
            fives: cnt.fives + 1,
            ..cnt
        },
        6 => Cnt {
            sixs: cnt.sixs + 1,
            ..cnt
        },
        _ => cnt,
    })
}

fn par_fun_dice_count(dices: &[u8]) -> Cnt {
    dices
        .par_iter()
        .map(|d| match d {
            1 => Cnt {
                ones: 1,
                ..Cnt::default()
            },
            2 => Cnt {
                twos: 1,
                ..Cnt::default()
            },
            3 => Cnt {
                threes: 1,
                ..Cnt::default()
            },
            4 => Cnt {
                fours: 1,
                ..Cnt::default()
            },
            5 => Cnt {
                fives: 1,
                ..Cnt::default()
            },
            6 => Cnt {
                sixs: 1,
                ..Cnt::default()
            },
            _ => {Cnt::default()},
        })
        .reduce(
            || Cnt::default(),
            |cnt, n| -> Cnt {match n {
                Cnt {
                    ones: 1,
                    twos: 0,
                    threes: 0,
                    fours: 0,
                    fives: 0,
                    sixs: 0,
                } => Cnt {
                    ones: cnt.ones + 1,
                    ..cnt
                },
                Cnt {
                    ones: 0,
                    twos: 1,
                    threes: 0,
                    fours: 0,
                    fives: 0,
                    sixs: 0,
                } => Cnt {
                    twos: cnt.twos + 1,
                    ..cnt
                },
                Cnt {
                    ones: 0,
                    twos: 0,
                    threes: 1,
                    fours: 0,
                    fives: 0,
                    sixs: 0,
                } => Cnt {
                    threes: cnt.threes + 1,
                    ..cnt
                },
                Cnt {
                    ones: 0,
                    twos: 0,
                    threes: 0,
                    fours: 1,
                    fives: 0,
                    sixs: 0,
                } => Cnt {
                    fours: cnt.fours + 1,
                    ..cnt
                },
                Cnt {
                    ones: 0,
                    twos: 0,
                    threes: 0,
                    fours: 0,
                    fives: 1,
                    sixs: 0,
                } => Cnt {
                    fives: cnt.fives + 1,
                    ..cnt
                },
                Cnt {
                    ones: 0,
                    twos: 0,
                    threes: 0,
                    fours: 0,
                    fives: 0,
                    sixs: 1,
                } => Cnt {
                    sixs: cnt.sixs + 1,
                    ..cnt
                },
                _ => Cnt {
                    ones: cnt.ones + n.ones,
                    twos: cnt.twos + n.twos,
                    threes: cnt.threes + n.threes,
                    fours: cnt.fours + n.fours,
                    fives: cnt.fives + n.fives,
                    sixs: cnt.sixs + n.sixs,
                },
            }},
        )
}
