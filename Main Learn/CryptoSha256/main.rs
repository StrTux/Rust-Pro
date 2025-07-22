use sha2::{Sha256, Digest};

fn find_hash_with_prefix(prefix: &str) -> (u32, String) {
    let mut input = 569193;
    loop {
        let input_str = format!("Ashish03{}", input);
    
    
        let mut hasher = Sha256::new();
    
        hasher.update(input_str.as_bytes());
    
        let hash = hasher.finalize();
    
        let hash_str = format!("{:x}", hash);

        if hash_str.starts_with(prefix) {
            return (input, hash_str);
        }
        input += 1;
    }
}

fn main() {
    let result = find_hash_with_prefix("00000");
    println!("Input: {}", result.0);
    println!("Hash: {}", result.1);
}
