use sha2::{Digest, Sha256};

pub fn hasher(inputs: &[&String]) -> String {
    let mut hasher = Sha256::new();
    for input in inputs {
        hasher.update(input);
    }
    let result = hasher.finalize();

    return format!("{:X}", result);
}
