use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;
use std::{default, sync::Arc};
use std::thread;
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

fn main() {
    // Roll some D6 dices
    let mut rng = rand::thread_rng();
    let dices: Vec<u8> = (0..100_000_000)
        .map(|_| Uniform::from(1..=6).sample(&mut rng))
        .collect();

    // Imperative dice count
    let time = Instant::now();
    dbg!(imp_dice_count(&dices));
    println!("imp_dice_count {:.1?}", time.elapsed());

    // Functional dice count
    let time = Instant::now();
    dbg!(fun_dice_count(&dices));
    println!("fun_dice_count {:.1?}", time.elapsed());

    // Parallell functional dice count. Embarrassing code to solve embarrassing parallel problem.
    let time = Instant::now();
    dbg!(par_fun_dice_count(&dices));
    println!("par_fun_dice_count {:.1?}", time.elapsed());

    // Multiple scans. Probably uses SIMD?
    let time = Instant::now();
    println!("ones: {}", dices.iter().filter(|&x| *x == 1).count());
    println!("twos: {}", dices.iter().filter(|&x| *x == 2).count());
    println!("threes: {}", dices.iter().filter(|&x| *x == 3).count());
    println!("fours: {}", dices.iter().filter(|&x| *x == 4).count());
    println!("fives: {}", dices.iter().filter(|&x| *x == 5).count());
    println!("sixs: {}", dices.iter().filter(|&x| *x == 6).count());
    println!("multiscan fun count {:.1?}", time.elapsed());

    // Bytecount crate
    let time = Instant::now();
    println!("bytecount ones:{}", bytecount::count(&dices, 1u8));
    println!("bytecount twos:{}", bytecount::count(&dices, 2u8));
    println!("bytecount threes:{}", bytecount::count(&dices, 3u8));
    println!("bytecount fours:{}", bytecount::count(&dices, 4u8));
    println!("bytecount fives:{}", bytecount::count(&dices, 5u8));
    println!("bytecount sixs:{}", bytecount::count(&dices, 6u8));
    println!("bytecount {:.1?}", time.elapsed());

    // OS threaded Bytecount crate
    let time = Instant::now();
    let dices_p = Arc::new(dices);
    let mut cnt = Arc::new(Cnt::default());
    let mut cnt_cloned = cnt.clone();
    let dices_p_cloned = dices_p.clone();
    println!("bytecount arc test {}:{}", 1, bytecount::count(&dices_p_cloned, 1u8));
    /*
    for n in 1..7u8 {
        println!("Starting thread:{}",n);
        let dices_p = dices_p.clone();
        thread::spawn(move || {
            println!("Hello from OS thread");
            //println!("bytecount {}:{}", n, bytecount::count(&dices_p_cloned, n));
            cnt_cloned.ones = bytecount::count(&dices_p, 1u8) as u32;
        });
        
    }
    println!("bytecount {:.1?}", time.elapsed());
*/
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
            _ => Cnt::default(),
        })
        .reduce(
            || Cnt::default(),
            |cnt, n| -> Cnt {
                match n {
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
                }
            },
        )
}
