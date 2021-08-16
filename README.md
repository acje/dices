# Dices
Rolling 100M D6 dices and counting them

RUSTFLAGS="-C target-cpu=native" cargo run --release

## Result, using i5-8250 (quad core, vm):

Single pass serial imp_dice_count 848.6ms

Single pass serial fun_dice_count 853.4ms

Rayon single pass parallell par_fun_dice_count 549.4ms

6 scans serial (can optimize to SIMD) multi_fun_dice_count 108.2ms

6 scans serial dice_bytecount 65.7ms

6 scans threaded_dice_bytecount 30.5ms

## Results, using AMD 5600X (6-core, bare metal)

Single pass serial imp_dice_count 570.8ms

Single pass serial fun_dice_count 571.7ms

Rayon single pass parallell par_fun_dice_count 135.8ms

6 scans serial (can optimize to SIMD) multi_fun_dice_count 48.5ms

6 scans serial dice_bytecount 24.4ms

6 scans threaded_dice_bytecount 5.3ms
