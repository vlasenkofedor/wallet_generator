use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::Keccak256;
use sha2::{Sha256, Digest};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use rand::{rng, RngCore};
use rayon;

fn generate_tron_address() -> (SecretKey, String) {
    let secp = Secp256k1::new();
    let mut rng = rng();
    let mut private_bytes = [0u8; 32];
    rng.fill_bytes(&mut private_bytes);
    let private_key = SecretKey::from_slice(&private_bytes).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_secret_key(&secp, &private_key);
    let pubkey_bytes = public_key.serialize_uncompressed();
    let mut hasher = Keccak256::new();
    hasher.update(&pubkey_bytes[1..]);
    let hash = hasher.finalize();
    let address_bytes = &hash[12..];
    let mut address_with_prefix = vec![0x41];
    address_with_prefix.extend_from_slice(address_bytes);
    let mut sha2 = Sha256::new();
    sha2.update(&address_with_prefix);
    let first_sha = sha2.finalize_reset();
    sha2.update(&first_sha);
    let checksum = &sha2.finalize()[..4];
    let mut full_bytes = address_with_prefix;
    full_bytes.extend_from_slice(checksum);
    let address = bs58::encode(full_bytes).into_string();

    (private_key, address)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <suffix> [case-sensitive]", args[0]);
        eprintln!("  suffix:         The suffix to search for");
        eprintln!("  case-sensitive: (optional) 'true' to enable case sensitivity");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} abc           # Case-insensitive search", args[0]);
        eprintln!("  {} Abc true      # Case-sensitive search", args[0]);
        std::process::exit(1);
    }

    let suffix = &args[1];
    let case_sensitive = args.len() == 3 && args[2].eq_ignore_ascii_case("true");

    // Подготовка суффикса в зависимости от режима
    let suffix_normalized = if case_sensitive {
        suffix.clone()
    } else {
        suffix.to_lowercase()
    };

    println!("Searching for TRON address ending with: '{}'", suffix);
    println!("Case sensitivity: {}", if case_sensitive { "enabled" } else { "disabled" });

    let found = Arc::new(AtomicBool::new(false));

    rayon::scope(|s| {
        for _ in 0..rayon::current_num_threads() {
            let suffix_clone = suffix_normalized.clone();
            let found_clone = Arc::clone(&found);
            let case_mode = case_sensitive;

            s.spawn(move |_| {
                while !found_clone.load(Ordering::Relaxed) {
                    let (private_key, address) = generate_tron_address();

                    // Проверяем совпадение с учетом выбранного режима
                    let matches = if case_mode {
                        address.ends_with(&suffix_clone)
                    } else {
                        address.to_ascii_lowercase().ends_with(&suffix_clone)
                    };

                    if matches {
                        found_clone.store(true, Ordering::Relaxed);
                        println!("\nFound matching address!");
                        println!("Address: {}", address);
                        println!("Private Key: {}", hex::encode(private_key.secret_bytes()));
                        break;
                    }
                }
            });
        }
    });
}