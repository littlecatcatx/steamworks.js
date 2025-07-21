use napi::bindgen_prelude::Error;
use napi_derive::napi;
use steamworks::AppId;
use steamworks::Client;
use steamworks::SteamAPIInitError;

pub mod client;
pub mod logger;

#[macro_use]
extern crate lazy_static;

use crate::logger::log_function_call;

#[napi]
pub fn init(app_id: Option<u32>) -> Result<(), Error> {
    let result = (|| {
        if client::has_client() {
            client::drop_client();
        }

        let steam_client = app_id
            .map(|app_id| Client::init_app(AppId(app_id)))
            .unwrap_or_else(Client::init)
            .map_err(|e| match e {
                SteamAPIInitError::FailedGeneric(msg)
                | SteamAPIInitError::NoSteamClient(msg)
                | SteamAPIInitError::VersionMismatch(msg) => Error::from_reason(msg),
            })?;

        steam_client.user_stats().request_current_stats();

        client::set_client(steam_client);
        Ok(())
    })();

    log_function_call("init", &[&app_id], &result.is_ok());
    result
}

#[napi]
pub fn restart_app_if_necessary(app_id: u32) -> bool {
    let result = steamworks::restart_app_if_necessary(AppId(app_id));
    log_function_call("restart_app_if_necessary", &[&app_id], &result);
    result
}

#[napi]
pub fn run_callbacks() {
    client::get_client().run_callbacks();
    log_function_call("run_callbacks", &[], &"()");
}

pub mod api;
