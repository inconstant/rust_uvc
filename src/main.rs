mod problems;

fn main() {
    use std::io;

    let buffer = get_stdin().unwrap();
    let n = buffer.trim().parse::<u64>().unwrap();

    //println!("{:?}", problems::prob1::multiples_of_3_and_5(n));
    //println!("{:?}", problems::prob2::even_fibonacci_numbers(n));
    //println!("{:?}", problems::prob3::largest_prime_factor(n));
    println!("{:?}", problems::prob4::largest_palindrome_product());

    fn get_stdin() -> io::Result<String> {
        let mut buffer = String::new();
        try!(io::stdin().read_line(&mut buffer));
        Ok(buffer)
    }
}
