pub fn largest_palindrome_product() -> u32 {
    let mut palindrome = 0;
    for x in 100..999 {
        for y in x..999 {
            if check_palindrome(x*y) {
                palindrome =  if palindrome < x*y { x*y } else { palindrome }
            }
        }
    }
    return palindrome;
}

fn check_palindrome(n: u32) -> bool {
    let s: String = n.to_string();
    let c = s.chars().collect::<Vec<char>>();
    for x in 0..s.len()/2 {
        if c[x] != c[c.len()-1-x] {
            return false
        }
    }
    return true;
}
