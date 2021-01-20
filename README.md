# dices
Rolling dices and counting them

RUSTFLAGS="-C target-cpu=native" cargo run --release

Result, using i7-7500U:

[src/main.rs:23] imp_dice_count(&dices) = Cnt {
    ones: 16668221,
    twos: 16665583,
    threes: 16660489,
    fours: 16668728,
    fives: 16667327,
    sixs: 16669652,
}

imp_dice_count 818.3ms

[src/main.rs:28] fun_dice_count(&dices) = Cnt {
    ones: 16668221,
    twos: 16665583,
    threes: 16660489,
    fours: 16668728,
    fives: 16667327,
    sixs: 16669652,
}

fun_dice_count 812.9ms

[src/main.rs:33] par_fun_dice_count(&dices) = Cnt {
    ones: 16668221,
    twos: 16665583,
    threes: 16660489,
    fours: 16668728,
    fives: 16667327,
    sixs: 16669652,
}

par_fun_dice_count 555.5ms

ones: 16668221
twos: 16665583
threes: 16660489
fours: 16668728
fives: 16667327
sixs: 16669652

multiscan fun count 80.0ms

bytecount ones:16668221
bytecount twos:16665583
bytecount threes:16660489
bytecount fours:16668728
bytecount fives:16667327
bytecount sixs:16669652

bytecount 50.7ms
