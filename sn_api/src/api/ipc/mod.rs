// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

/// Request module.
pub mod req;
/// Response module.
pub mod resp;

mod errors;

pub use self::errors::IpcError;
pub use self::req::{
    // AppExchangeInfo,
    AuthReq,
    // ContainersReq,
    IpcReq,
    // Permission,
    // ShareMap,
    // ShareMapReq,
};
pub use self::resp::{AuthGranted, IpcResp};

use bincode::{deserialize, serialize};
use data_encoding::BASE32_NOPAD;
// #[cfg(any(test, feature = "testing"))]
// use rand::{self};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, net::SocketAddr, u32};

use log::{debug, info};

/// `QuicP2P` bootstrap info, shared from Authenticator to apps.
pub type BootstrapConfig = HashSet<SocketAddr>;

/// IPC message.
#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum IpcMsg {
    /// Request.
    Req {
        /// Request ID.
        req_id: u32,
        /// Request.
        request: IpcReq,
    },
    /// Response.
    Resp {
        /// Request ID.
        req_id: u32,
        /// Response.
        response: IpcResp,
    },
    /// Revoked.
    Revoked {
        /// Application ID.
        app_id: String,
    },
    /// Generic error like couldn't parse IpcMsg etc.
    Err(IpcError),
}

/// Encode `IpcMsg` into string, using base32 encoding.
pub fn encode_msg(msg: &IpcMsg) -> Result<String, IpcError> {
    // We also add a multicodec compatible prefix. For more details please follow
    // https://github.com/multiformats/multicodec/blob/master/table.csv
    Ok(format!("b{}", BASE32_NOPAD.encode(&serialize(&msg)?)))
}

/// Decode `IpcMsg` encoded with base32 encoding.
pub fn decode_msg(encoded: &str) -> Result<IpcMsg, IpcError> {
    info!("ENCODED MSG STRING: {:?}", encoded);
    let mut chars = encoded.chars();
    let decoded = match chars.next().ok_or(IpcError::InvalidMsg)? {
        // Encoded as base32
        'b' | 'B' => BASE32_NOPAD.decode(chars.as_str().as_bytes())?,
        // Fail if not encoded as base32
        _ => {
            debug!("This didn't start with B, wth...");
            return Err(IpcError::EncodeDecodeError);
        }
    };

    let msg: IpcMsg = deserialize(&decoded)?;

    Ok(msg)
}

/// Generate unique request ID.
pub fn gen_req_id() -> u32 {
    use rand::Rng;
    // Generate the number in range 1..MAX inclusive.
    rand::thread_rng().gen_range(0, u32::max_value()) + 1
}
