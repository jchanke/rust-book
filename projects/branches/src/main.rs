fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let tests = [
        // Check: fahrenheit_to_celsius
        {
            println!("65F is {:.2}C", fahrenheit_to_celsius(65.0));
            true
        },
        fahrenheit_to_celsius(-40.0) == celsius_to_fahrenheit(-40.0),
        // Check: Fibonacci
        {
            for i in 0..10 {
                println!("fibonacci({i}) = {}", fibonacci(i))
            }
            true
        },
        {
            println!("fibonacci(-1) = {}", fibonacci(-1));
            true
        },
    ];

    for test in tests {
        if !test {
            println!("failed test!");
        }
    }
    println!("passed all tests!")
}

fn fahrenheit_to_celsius(t: f32) -> f32 {
    (t - 32.0) / 1.8
}

fn celsius_to_fahrenheit(t: f32) -> f32 {
    (1.8 * t) + 32.0
}

fn fibonacci(n: i32) -> i32 {
    let (mut a, mut b) = (0, 1);
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        for _ in 1..n {
            (a, b) = (b, a + b);
        }
        b
    }
}
