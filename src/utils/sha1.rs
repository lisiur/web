use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub fn sha1(input: &str) -> String {
    // create a Sha1 object
    let mut hasher = Sha1::new();

    // write input message
    hasher.input_str(input);

    // read hash digest
    hasher.result_str()
}
