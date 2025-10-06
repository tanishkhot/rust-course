pub fn zeros() -> [u32; 100] {
    let arr:[u32; 100] = [0 ; 100];
    return arr;
}

pub fn first_3(s: &[u32]) -> &[u32] {
    return &s[0..3];
}

pub fn last_3(s: &[u32]) -> &[u32] {
    let n = s.len();
    return &s[n-3..n];
}
