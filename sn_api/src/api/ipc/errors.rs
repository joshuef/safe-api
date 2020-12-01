// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use serde::{Deserialize, Serialize};
//use threshold_crypto::error::FromBytesError;

/// Ipc error.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum IpcError {
    /// Authentication denied.
    AuthDenied,
    /// Invalid IPC message.
    InvalidMsg,
    /// Generic encoding / decoding failure.
    EncodeDecodeError,
    /// App is already authorised.
    AlreadyAuthorised,
    /// App is not registered.
    UnknownApp,
    /// Unexpected error.
    Unexpected(String),
}

/*
impl From<serde_json::Error> for IpcError {
    fn from(error: serde_json::Error) -> Self {
        Self::EncodeDecodeError
    }
}

impl From<SendError> for IpcError {
    fn from(error: SendError) -> Self {
        Self::Unexpected(error.to_string())
    }
}

impl From<Utf8Error> for IpcError {
    fn from(_err: Utf8Error) -> Self {
        Self::EncodeDecodeError
    }
}

impl<'a> From<&'a str> for IpcError {
    fn from(s: &'a str) -> Self {
        Self::Unexpected(s.to_string())
    }
}

impl From<String> for IpcError {
    fn from(s: String) -> Self {
        Self::Unexpected(s)
    }
}

impl From<FromBytesError> for IpcError {
    fn from(err: FromBytesError) -> Self {
        Self::from(err.to_string())
    }
}
*/
