use rand::Rng;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

pub fn random_str(len: usize) -> String {
    let mut rng = rand::thread_rng();

    let string: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..len);
            CHARSET[idx] as char
        })
        .collect();

    string
}
