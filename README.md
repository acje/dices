# dices
Rolling dices and counting them

RUSTFLAGS="-C target-cpu=native" cargo run --release

Result, using i5-8250:

[src/main.rs:25] imp_dice_count(&dices) = Cnt {
    ones: 16678078,
    twos: 16661966,
    threes: 16662117,
    fours: 16663610,
    fives: 16667468,
    sixs: 16666761,
}
imp_dice_count 854.9ms

[src/main.rs:30] fun_dice_count(&dices) = Cnt {
    ones: 16678078,
    twos: 16661966,
    threes: 16662117,
    fours: 16663610,
    fives: 16667468,
    sixs: 16666761,
}
fun_dice_count 875.6ms

[src/main.rs:35] par_fun_dice_count(&dices) = Cnt {
    ones: 16678078,
    twos: 16661966,
    threes: 16662117,
    fours: 16663610,
    fives: 16667468,
    sixs: 16666761,
}
par_fun_dice_count 499.5ms

ones: 16678078
twos: 16661966
threes: 16662117
fours: 16663610
fives: 16667468
sixs: 16666761
multiscan fun count 117.3ms

bytecount ones:16678078
bytecount twos:16661966
bytecount threes:16662117
bytecount fours:16663610
bytecount fives:16667468
bytecount sixs:16666761
bytecount 85.0ms

Threaded:Cnt { ones: 16678078, twos: 16661966, threes: 16662117, fours: 16663610, fives: 16667468, sixs: 16666761 }
Threaded bytecount 38.5ms
