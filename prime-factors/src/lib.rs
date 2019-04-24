pub fn factors(number: u64) -> Vec<u64> {
    let mut number = number ; 
    let mut prime_numbers = vec! [];
    let mut dividen = 2;

    while number > 1 {
        while number % dividen == 0 {
            prime_numbers.push(dividen);
            number /= dividen;
        }
        dividen += 1;
    }

    return prime_numbers;
}

 