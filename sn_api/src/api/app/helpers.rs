// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

#[cfg(feature = "app")]
use crate::{
    api::{
        authenticator::AuthResponseType,
        ipc::{resp::AuthGranted, BootstrapConfig, IpcMsg, IpcResp},
    },
    Error, Result,
};
use chrono::{DateTime, SecondsFormat, Utc};

use sn_data_types::{Error as SafeNdError, Money, PublicKey};
use std::str::{self, FromStr};
use std::time;
use xor_name::XorName;

/// The conversion from coin to raw value
const COIN_TO_RAW_CONVERSION: u64 = 1_000_000_000;
// The maximum amount of safecoin that can be represented by a single `Money`
const MAX_COINS_VALUE: u64 = (u32::max_value() as u64 + 1) * COIN_TO_RAW_CONVERSION - 1;

#[allow(dead_code)]
pub fn vec_to_hex(hash: Vec<u8>) -> String {
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn xorname_to_hex(xorname: &XorName) -> String {
    xorname.0.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn pk_to_hex(pk: &PublicKey) -> String {
    let xorname = XorName::from(*pk);
    xorname_to_hex(&xorname)
}

// #[allow(dead_code)]
// pub fn pk_from_hex(hex_str: &str) -> Result<PublicKey> {
//     let pk_bytes = parse_hex(&hex_str);
//     let mut pk_bytes_array: [u8; PK_SIZE] = [0; PK_SIZE];
//     pk_bytes_array.copy_from_slice(&pk_bytes[..PK_SIZE]);
//     PublicKey::from_bytes(pk_bytes_array)
//         .map_err(|_| Error::InvalidInput("Invalid public key bytes".to_string()))
// }

pub fn parse_coins_amount(amount_str: &str) -> Result<Money> {
    Money::from_str(amount_str).map_err(|err| {
        match err {
            SafeNdError::ExcessiveValue => Error::InvalidAmount(format!(
                "Invalid safecoins amount '{}', it exceeds the maximum possible value '{}'",
                amount_str, Money::from_nano(MAX_COINS_VALUE)
            )),
            SafeNdError::LossOfPrecision => {
                Error::InvalidAmount(format!("Invalid safecoins amount '{}', the minimum possible amount is one nano coin (0.000000001)", amount_str))
            }
            SafeNdError::FailedToParse(msg) => {
                Error::InvalidAmount(format!("Invalid safecoins amount '{}' ({})", amount_str, msg))
            },
            _ => Error::InvalidAmount(format!("Invalid safecoins amount '{}'", amount_str)),
        }
    })
}

pub fn decode_auth_response_ipc_msg(ipc_msg: &str) -> Result<AuthResponseType> {
    let msg = serde_json::from_str(ipc_msg)
        .map_err(|e| Error::InvalidInput(format!("Failed to decode the credentials: {:?}", e)))?;
    match msg {
        IpcMsg::Resp(IpcResp::Auth(Ok(authgranted))) => {
            Ok(AuthResponseType::Registered(authgranted))
        }
        IpcMsg::Resp(IpcResp::Auth(Err(e))) => Err(Error::AuthError(format!("{:?}", e))),
        IpcMsg::Resp(IpcResp::Unregistered(Ok(config))) => {
            Ok(AuthResponseType::Unregistered(config))
        }
        IpcMsg::Resp(IpcResp::Unregistered(Err(e))) => Err(Error::AuthError(format!("{:?}", e))),
        other => Err(Error::AuthError(format!("{:?}", other))),
    }
}

pub fn systemtime_to_rfc3339(t: &time::SystemTime) -> String {
    let datetime: DateTime<Utc> = t.clone().into();
    datetime.to_rfc3339_opts(SecondsFormat::Secs, true)
}

pub fn gen_timestamp_secs() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}
