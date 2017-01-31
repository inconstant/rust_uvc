pub fn even_fibonacci_numbers(lim: u64) -> u64 {
    tail_even_fibonacci_numbers(lim, 1, 0, 0)
}

fn tail_even_fibonacci_numbers(lim: u64, current: u64, before: u64, att: u64) -> u64 {
    if current > lim {
        return att
    }

    let n_current = current + before;
    let n_att = if current%2 == 0 {
        att + current
    } else {
        att
    };

    tail_even_fibonacci_numbers(lim, n_current, current, n_att)
}
