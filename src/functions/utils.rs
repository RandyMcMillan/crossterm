use bech32::{ToBase32, Variant};
use rand::Rng;

pub enum Prefix {
    Npub,
    Nsec,
}

// Display 'trait' needed for enum "to_string()"
impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prefix::Npub => write!(f, "npub"),
            Prefix::Nsec => write!(f, "nsec"),
        }
    }
}
trait VecSeekOrAppend<T:PartialEq>:Sized {
    fn get_or_insert(&mut self, item: T) -> &T;
}

impl<T> VecSeekOrAppend<T> for Vec<T>
    where T: PartialEq + Clone {

    fn get_or_insert(&mut self, item: T) -> &T {
        if !self.contains(&item) {
            self.push(item.clone());
        }
        for i in self.iter() {
            if i == &item {
                return i;
            }
        }
        unreachable!();
    }
}
/// Converts a hex encoded string to bech32 format for given a Prefix (hrp)
pub fn bech32_encode(hrp: Prefix, hex_key: &String) -> String {
    bech32::encode(
        &hrp.to_string(),
        hex::decode(hex_key)
            .expect(&("could not decode provided key/note id=".to_owned() + hex_key))
            .to_base32(),
        Variant::Bech32,
    )
    .expect("Could not bech32-encode data")
}

pub fn random_hash() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    hex::encode(bytes)
}
#[test]
fn appends_ok() {
    let mut r = vec![1,2,3];
    assert_eq!(r.get_or_insert(3), &3);
    assert_eq!(r.len(), 3);
    assert_eq!(r.get_or_insert(4), &4);
    assert_eq!(r.len(), 4);
}
