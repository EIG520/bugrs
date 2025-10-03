pub const fn parse_usize_const(s: &str) -> usize {
    let mut n = 0;
    let it = s.as_bytes();

    let mut i = it.len();
    while i > 0 {
        i -= 1;

        n *= 10;
        n += it[i] as usize - 48;
    }

    n
}