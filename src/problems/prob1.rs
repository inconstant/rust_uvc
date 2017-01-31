pub fn multiples_of_3_and_5(n: u32) -> u64{
    tail_multiples_of_3_and_5(n, 0)
}

fn tail_multiples_of_3_and_5(n: u32, att: u64) -> u64 {
    if n == 0 {
        return att;
    }
    let n_att = if n%3==0 || n%5==0 {
        att + n as u64
    } else {
        att
    };
    tail_multiples_of_3_and_5(n-1, n_att)
}
