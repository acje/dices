# dices
Rolling dices and counting them

RUSTFLAGS="-C target-cpu=native" cargo run --release

Result, using i5-8250:

[src/main.rs:23] imp_dice_count(&dices) = Cnt {
    ones: 16673061,
    twos: 16666241,
    threes: 16661574,
    fours: 16666719,
    fives: 16662423,
    sixs: 16669982,
}
Single pass serial imp_dice_count 940.7ms


[src/main.rs:28] fun_dice_count(&dices) = Cnt {
    ones: 16673061,
    twos: 16666241,
    threes: 16661574,
    fours: 16666719,
    fives: 16662423,
    sixs: 16669982,
}
Single pass serial fun_dice_count 898.4ms


[src/main.rs:33] par_fun_dice_count(&dices) = Cnt {
    ones: 16673061,
    twos: 16666241,
    threes: 16661574,
    fours: 16666719,
    fives: 16662423,
    sixs: 16669982,
}
Rayon single pass parallell par_fun_dice_count 576.0ms
[src/main.rs:38] multi_fun_dice_count(&dices) = Cnt {
    ones: 16673061,
    twos: 16666241,
    threes: 16661574,
    fours: 16666719,
    fives: 16662423,
    sixs: 16669982,
}
6 scanns serial /w SIMD? multi_fun_dice_count 93.6ms


[src/main.rs:43] dice_bytecount(&dices) = Cnt {
    ones: 16673061,
    twos: 16666241,
    threes: 16661574,
    fours: 16666719,
    fives: 16662423,
    sixs: 16669982,
}
6 scans serial dice_bytecount 60.5ms


[src/main.rs:49] threaded_dice_bytecount(&dices) = Cnt {
    ones: 16673061,
    twos: 16666241,
    threes: 16661574,
    fours: 16666719,
    fives: 16662423,
    sixs: 16669982,
}
6 scanns threaded_dice_bytecount 36.7ms
