# dices
Rolling dices and counting them

RUSTFLAGS="-C target-cpu=native" cargo run --release

Result, using i5-8250:

Single pass serial imp_dice_count 940.7ms

Single pass serial fun_dice_count 898.4ms

Rayon single pass parallell par_fun_dice_count 576.0ms

6 scanns serial /w SIMD? multi_fun_dice_count 93.6ms

6 scans serial dice_bytecount 60.5ms

6 scanns threaded_dice_bytecount 36.7ms
