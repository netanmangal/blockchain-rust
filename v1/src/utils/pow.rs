use super::hasher;

pub fn proof_of_work(
    index: &String,
    creator_address: &String,
    timestamp: &String,
    prev_block_hash: &String,
    merkle_root_hash: &String,
) -> u32 {
    let mut nonce: u32 = 0;
    let mut hash: String = hasher::hasher(&[
        index,
        creator_address,
        timestamp,
        &nonce.to_string(),
        prev_block_hash,
        merkle_root_hash,
    ]);

    while hash.chars().nth(0).unwrap() != '0' {
        println!("{}", hash.chars().nth(0).unwrap());
        println!("{}", hash);
        nonce += 1;
        hash = hasher::hasher(&[
            index,
            creator_address,
            timestamp,
            &nonce.to_string(),
            prev_block_hash,
            merkle_root_hash,
        ]);
    }

    return nonce;
}

// let block_hash: String = hasher::hasher(&[
//     &count.to_string(),
//     creator_address,
//     &timestamp.to_string(),
//     &nonce.to_string(),
//     prev_block_hash,
//     &merkle_root_hash,
// ]);
