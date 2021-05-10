use serde::{self, Serialize};

use bip39::{MnemonicType, Mnemonic, Language};
use std::{io,convert::TryFrom};
use sp_core::{
    sr25519,
    Pair, hexdisplay::HexDisplay,
    crypto::{Ss58Codec, Ss58AddressFormat},
};
use sp_runtime::{MultiSigner, traits::IdentifyAccount};

use std::str;
use x25519_dalek::{PublicKey, StaticSecret};
use crate::error::Result;
use crate::json::JsonPrettyFormatter;
use std::str::FromStr;

pub type PublicFor<P> = <P as sp_core::Pair>::Public;
pub type SeedFor<P> = <P as sp_core::Pair>::Seed;


pub fn format_seed<P: sp_core::Pair>(seed: SeedFor<P>) -> String {
    format!("0x{}", HexDisplay::from(&seed.as_ref()))
}


fn format_public_key<P: sp_core::Pair>(public_key: PublicFor<P>) -> String {
    format!("0x{}", HexDisplay::from(&public_key.as_ref()))
}

fn format_address<Pair>(public_key: PublicFor<Pair>, network_override: Ss58AddressFormat) -> String where
Pair: sp_core::Pair,
Pair::Public: Into<MultiSigner>, {
    format!("{}", &public_key.into().into_account().to_ss58check_with_version(network_override))
}

pub fn generate(words: &str, format: &str, amount: i32, network: &str) -> Result<()> {

    let network_override = Ss58AddressFormat::from_str(network)?;

    match format {
        "json" => json_writer(words, network_override),
        "csv" => csv_writer(words, amount,network_override),
        _ => Ok(()),
    };
    Ok(())
}

pub fn json_writer(words: &str, network_override: Ss58AddressFormat) -> Result<()> {
    let words = words.parse::<usize>().unwrap_or(12);

    let mnemonic = Mnemonic::new(MnemonicType::for_word_count(words).unwrap(), Language::English);

    if let Ok((pair, seed)) = sr25519::Pair::from_phrase(mnemonic.phrase(), None) {
        let obj = json!({
					"secret_phrase": mnemonic.phrase(),
					"secret_seed": format_seed::<sr25519::Pair>(seed),
					"public_key": format_public_key::<sr25519::Pair>(pair.public().clone()),
					"account_id": format_public_key::<sr25519::Pair>(pair.public().clone()),
					"ss58": format_address::<sr25519::Pair>(pair.public().clone(),network_override),
				});

        let buf = Vec::new();
        let formatter = JsonPrettyFormatter::new();
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        obj.serialize(&mut ser)?;
        println!("{}", String::from_utf8(ser.into_inner())?);
    }
    Ok(())
}

pub fn csv_writer(words: &str, amount: i32, network_override: Ss58AddressFormat) -> Result<()> {
    let words = words.parse::<usize>().unwrap_or(12);


    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b',')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(io::stdout());

    wtr.write_record(&[
        "secret_phrase",
        "secret_seed",
        "public_key",
        "account_id",
        "ss58",
    ])?;

    for _ in 0..amount {
        let mnemonic = Mnemonic::new(MnemonicType::for_word_count(words).unwrap(), Language::English);
        if let Ok((pair, seed)) = sr25519::Pair::from_phrase(mnemonic.phrase(), None) {
            wtr.write_record(&[
                mnemonic.phrase(),
                format_seed::<sr25519::Pair>(seed).as_str(),
                format_public_key::<sr25519::Pair>(pair.public().clone()).as_str(),
                format_public_key::<sr25519::Pair>(pair.public().clone()).as_str(),
                format_address::<sr25519::Pair>(pair.public().clone(),network_override).as_str(),
            ])?;
        }
    }


    wtr.flush()?;
    Ok(())
}