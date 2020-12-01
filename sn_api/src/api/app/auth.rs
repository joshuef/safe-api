// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use super::{
    common::send_authd_request,
    constants::{SN_AUTHD_ENDPOINT_HOST, SN_AUTHD_ENDPOINT_PORT},
    Safe,
};
use crate::api::ipc::IpcMsg;
use crate::{Error, Result};
use log::{debug, info};
use serde_json::json;

// Method for requesting application's authorisation
const SN_AUTHD_METHOD_AUTHORISE: &str = "authorise";

impl Safe {
    // Generate an authorisation request string and send it to a SAFE Authenticator.
    // It returns the credentials necessary to connect to the network, encoded in a single string.
    pub async fn auth_app(
        app_id: &str,
        app_name: &str,
        app_vendor: &str,
        endpoint: Option<&str>,
    ) -> Result<String> {
        // TODO: allow to accept all type of permissions to be passed as args to this API
        info!("Sending authorisation request to SAFE Authenticator...");

        let request = IpcMsg::new_auth_req(app_id, app_name, app_vendor);
        let auth_req_str = request.to_string()?;
        debug!(
            "Authorisation request generated successfully: {}",
            auth_req_str
        );

        // Send the auth request to authd and obtain the response
        let auth_res = send_app_auth_req(&auth_req_str, endpoint).await?;

        // Check if the app has been authorised
        match IpcMsg::from_string(&auth_res) {
            Ok(IpcMsg::Resp(_ipc_resp)) => {
                info!("Application was authorised: {:?}", auth_res);
                Ok(auth_res)
            }
            Ok(other) => {
                info!("Unexpected messages received: {:?}", other);
                Err(Error::AuthError(format!(
                    "Application was not authorised, unexpected response was received: {:?}",
                    other
                )))
            }
            Err(e) => {
                info!("Application was not authorised");
                Err(Error::AuthError(format!(
                    "Application was not authorised: {:?}",
                    e
                )))
            }
        }
    }

    // Connect to the SAFE Network using the provided auth credentials
    pub async fn connect(&mut self, auth_credentials: Option<&str>) -> Result<()> {
        self.safe_client.connect(auth_credentials).await
    }
}

// Sends an authorisation request string to the SAFE Authenticator daemon endpoint.
// It returns the credentials necessary to connect to the network, encoded in a single string.
async fn send_app_auth_req(auth_req_str: &str, endpoint: Option<&str>) -> Result<String> {
    let authd_service_url = match endpoint {
        None => format!("{}:{}", SN_AUTHD_ENDPOINT_HOST, SN_AUTHD_ENDPOINT_PORT,),
        Some(endpoint) => endpoint.to_string(),
    };

    info!("Sending authorisation request to SAFE Authenticator...");
    let authd_response = send_authd_request::<String>(
        &authd_service_url,
        SN_AUTHD_METHOD_AUTHORISE,
        json!(auth_req_str),
    )
    .await?;

    info!("SAFE authorisation response received!");
    Ok(authd_response)
}
