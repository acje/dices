use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;
use std::{sync::Arc, thread, time::Instant};
#[derive(Debug, Default, Clone)]
struct Cnt {
    ones: usize,
    twos: usize,
    threes: usize,
    fours: usize,
    fives: usize,
    sixs: usize,
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
    println!("Single pass serial imp_dice_count {:.1?}", time.elapsed());

    // Single pass functional dice count
    let time = Instant::now();
    dbg!(fun_dice_count(&dices));
    println!("Single pass serial fun_dice_count {:.1?}", time.elapsed());

    // Parallell/rayon functional dice count. Embarrassing code to solve embarrassing parallel problem.
    let time = Instant::now();
    dbg!(par_fun_dice_count(&dices));
    println!(
        "Rayon single pass parallell par_fun_dice_count {:.1?}",
        time.elapsed()
    );

    // Multiple scans. Probably uses SIMD?
    let time = Instant::now();
    dbg!(multi_fun_dice_count(&dices));
    println!(
        "6 scans serial /w SIMD? multi_fun_dice_count {:.1?}",
        time.elapsed()
    );

    // Bytecount crate
    let time = Instant::now();
    dbg!(dice_bytecount(&dices));
    println!("6 scans serial dice_bytecount {:.1?}", time.elapsed());

    // OS threaded Bytecount crate
    let time = Instant::now();
    let dices = Arc::new(dices);
    dbg!(threaded_dice_bytecount(&dices));
    println!("6 scans threaded_dice_bytecount {:.1?}", time.elapsed());
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

fn multi_fun_dice_count(dices: &[u8]) -> Cnt {
    Cnt {
        ones: dices.iter().filter(|&x| *x == 1).count(),
        twos: dices.iter().filter(|&x| *x == 2).count(),
        threes: dices.iter().filter(|&x| *x == 3).count(),
        fours: dices.iter().filter(|&x| *x == 4).count(),
        fives: dices.iter().filter(|&x| *x == 5).count(),
        sixs: dices.iter().filter(|&x| *x == 6).count(),
    }
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

fn dice_bytecount(dices: &[u8]) -> Cnt {
    Cnt {
        ones: bytecount::count(&dices, 1u8),
        twos: bytecount::count(&dices, 2u8),
        threes: bytecount::count(&dices, 3u8),
        fours: bytecount::count(&dices, 4u8),
        fives: bytecount::count(&dices, 5u8),
        sixs: bytecount::count(&dices, 6u8),
    }
}

fn threaded_dice_bytecount(dices: &Arc<Vec<u8>>) -> Cnt {
    let dices_p = dices.clone();
    let ones = thread::spawn(move || bytecount::count(&dices_p, 1u8));
    let dices_p = dices.clone();
    let twos = thread::spawn(move || bytecount::count(&dices_p, 2u8));
    let dices_p = dices.clone();
    let threes = thread::spawn(move || bytecount::count(&dices_p, 3u8));
    let dices_p = dices.clone();
    let fours = thread::spawn(move || bytecount::count(&dices_p, 4u8));
    let dices_p = dices.clone();
    let fives = thread::spawn(move || bytecount::count(&dices_p, 5u8));
    let dices_p = dices.clone();
    let sixs = thread::spawn(move || bytecount::count(&dices_p, 6u8));
    Cnt {
        ones: ones.join().expect("Error counting ones"),
        twos: twos.join().expect("Error counting twos"),
        threes: threes.join().expect("Error counting threes"),
        fours: fours.join().expect("Error counting fours"),
        fives: fives.join().expect("Error counting fives"),
        sixs: sixs.join().expect("Error counting sixs"),
    }
}
