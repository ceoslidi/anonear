use rand::Rng;
use sha1::{Sha1, Digest};

pub struct Wallet {
    pub(crate) public_key: String,
    pub(crate) private_key: String
}

/// Generate Public Key and Private Key
pub fn create_wallet() -> Wallet {
    //!
    //! Return ```Wallet``` with Wallet and Mnemonic phrase
    //!
    //! ```no_run
    //! pub struct Wallet {
    //!     public_key: String,
    //!     private_key: String
    //! }
    //! ```
    //!
    //! Mnemonic contains 12 random words from the dictionary
    let alphabet = [
        "air",
        "animal",
        "answer",
        "area",
        "bird",
        "body",
        "book",
        "bottom",
        "boy",
        "brother",
        "car",
        "child"
    ];

    // Create new hasher object
    let mut hasher = Sha1::new();

    let mut private_key = "".to_string();

    // Generate mnemonic phrase with 12 words
    for _ in 0..12 {
        let index = rand::thread_rng().gen_range(1..12);

        let mut temp_phrase = private_key;
        let new_word = alphabet[index];

        temp_phrase.push_str(&new_word);
        temp_phrase.push_str(" ");

        private_key = temp_phrase;
    }

    // Decode Generic Array to String
    hasher.update(private_key.as_bytes());
    let generic_wallet = hasher.finalize();
    let public_key = format!(
        "{:x}",
        generic_wallet
    );

    let wallet = Wallet {
        public_key,
        private_key
    };

    return wallet;
}