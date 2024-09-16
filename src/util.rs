use rand::{rngs::OsRng, Rng};

pub fn random_string(len: usize, chars: &[char]) -> String {
    let mut rng = OsRng;
    let mut res = vec!['\0'; len];

    for _ in 0..len {
        let n = rng.gen_range(0..chars.len());
        res.push(chars[n]);
    }

    res.iter().collect()
}
