use hex::FromHexError;
use snafu::*;

use crate::{PublicKeyErrors};

#[derive(Debug, Snafu)]
pub enum AddressErrors {
    #[snafu(display("Could not decode Base58 string: {} {}", string, error))]
    Base58Decode {
      error: anyhow::Error,
      string: String
    },

    #[snafu(display("{}", error))]
    PublicKeyError {
      error: PublicKeyErrors
    },

    #[snafu(display("Could not parse hex {}: {}", hex, error))]
    ParseHex {
      hex: String,
      error: FromHexError
    },
}