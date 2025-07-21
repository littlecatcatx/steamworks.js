use napi::bindgen_prelude::BigInt;
use napi_derive::napi;
use steamworks::SteamId;

#[derive(Debug)]
#[napi(object)]
pub struct PlayerSteamId {
    pub steam_id64: BigInt,
    pub steam_id32: String,
    pub account_id: u32,
}

impl PlayerSteamId {
    pub(crate) fn from_steamid(steam_id: SteamId) -> Self {
        Self {
            steam_id64: steam_id.raw().into(),
            steam_id32: steam_id.steamid32(),
            account_id: steam_id.account_id().raw(),
        }
    }
}

#[napi]
pub mod localplayer {
    use super::PlayerSteamId;
    use crate::logger::log_function_call;

    #[napi]
    pub fn get_steam_id() -> PlayerSteamId {
        #[cfg(not(feature = "mock"))]
        let result = {
            let client = crate::client::get_client();
            let steam_id = client.user().steam_id();
            PlayerSteamId::from_steamid(steam_id)
        };
        #[cfg(feature = "mock")]
        let result = {
            use napi::bindgen_prelude::BigInt;
            PlayerSteamId {
                steam_id64: BigInt {
                    sign_bit: false,
                    words: vec![77561192047718298],
                },
                steam_id32: "STEAM_0:0:63223210".to_string(),
                account_id: 97242873,
            }
        };
        log_function_call("localplayer.get_steam_id", &[], &result);
        result
    }

    #[napi]
    pub fn get_name() -> String {
        #[cfg(not(feature = "mock"))]
        let result = {
            let client = crate::client::get_client();
            client.friends().name()
        };
        #[cfg(feature = "mock")]
        let result = "shadow".to_string();
        log_function_call("localplayer.get_name", &[], &result);
        result
    }

    #[napi]
    pub fn get_level() -> u32 {
        let result = {
            let client = crate::client::get_client();
            client.user().level()
        };
        log_function_call("localplayer.get_level", &[], &result);
        result
    }

    /// @returns the 2 digit ISO 3166-1-alpha-2 format country code which client is running in, e.g. "US" or "UK".
    #[napi]
    pub fn get_ip_country() -> String {
        let result = {
            let client = crate::client::get_client();
            client.utils().ip_country()
        };
        log_function_call("localplayer.get_ip_country", &[], &result);
        result
    }

    #[napi]
    pub fn set_rich_presence(key: String, value: Option<String>) {
        let client = crate::client::get_client();
        client.friends().set_rich_presence(&key, value.as_deref());
        log_function_call("localplayer.set_rich_presence", &[&key, &value], &"()");
    }
}
