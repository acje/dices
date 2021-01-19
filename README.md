# dices
Rolling dices and counting them

RUSTFLAGS="-C target-cpu=native" cargo run --release

result:

[src/main.rs:23] imp_dice_count(&dices) = Cnt {
    ones: 33337373,
    twos: 33337070,
    threes: 33336634,
    fours: 33332531,
    fives: 33327318,
    sixs: 33329074,
}

imp_dice_count 1.6s

[src/main.rs:28] fun_dice_count(&dices) = Cnt {
    ones: 33337373,
    twos: 33337070,
    threes: 33336634,
    fours: 33332531,
    fives: 33327318,
    sixs: 33329074,
}

fun_dice_count 1.6s

[src/main.rs:33] par_fun_dice_count(&dices) = Cnt {
    ones: 33337373,
    twos: 33337070,
    threes: 33336634,
    fours: 33332531,
    fives: 33327318,
    sixs: 33329074,
}

par_fun_dice_count 1.1s

ones: 33337373
twos: 33337070
threes: 33336634
fours: 33332531
fives: 33327318
sixs: 33329074

multiscan fun count 227.2ms
