//! Signs a stand-in object with a fixed key, prints the public key and the object's id, and checks
//! the signature before and after changing the body in its encoded bytes.
//!
//! Run it with `cargo run -p ledger --example sign`.

use std::error::Error;

use borsh::{BorshDeserialize, BorshSerialize};
use ledger::primitives::{Object, ObjectType, Signed, SigningKey};

/// A stand-in body until the real records exist, signed as a mark.
#[derive(BorshSerialize, BorshDeserialize)]
struct Note {
    text: String,
}

impl Object for Note {
    const TYPE: ObjectType = ObjectType::Mark;
}

fn main() -> Result<(), Box<dyn Error>> {
    let key = SigningKey::from_seed(&[7; 32]);
    let note = Note {
        text: String::from("Anna paid Mark 35 EUR"),
    };
    let signed = Signed::sign(note, &key)?;
    println!("public key  {}", signed.signer());
    println!("domain      {}", Note::TYPE.domain());
    show(&signed)?;

    // A signed object can't be edited, so change its bytes, as a peer could on the way.
    println!("\nAfter changing 35 to 36 in the encoded bytes:");
    let mut bytes = borsh::to_vec(&signed)?;
    // The body comes first, so its 35 is found first.
    let at = bytes
        .windows(2)
        .position(|pair| pair == b"35")
        .ok_or("no 35 in the encoded bytes")?;
    bytes[at + 1] = b'6';
    let changed: Signed<Note> = borsh::from_slice(&bytes)?;
    show(&changed)
}

fn show(signed: &Signed<Note>) -> Result<(), Box<dyn Error>> {
    println!("body        {}", signed.body().text);
    println!("id          {}", signed.id()?);
    match signed.verify() {
        Ok(()) => println!("verified"),
        Err(e) => println!("rejected: {e}"),
    }
    Ok(())
}
