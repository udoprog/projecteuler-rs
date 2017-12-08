fn run(mut number: u64) -> u64 {
    let mut factor = 1u64;

    let ceil = f64::ceil(f64::sqrt(number as f64)) as u64;

    for n in 2u64..ceil {
        while number % n == 0 {
            factor = n;
            number = number / n;
        }
    }

    factor
}

problem!{
    tests => [
        example => (29, run(13195)),
        q => (6857, run(600851475143)),
    ];
}
