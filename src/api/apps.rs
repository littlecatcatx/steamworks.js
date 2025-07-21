use super::localplayer::PlayerSteamId;
use napi_derive::napi;

#[napi]
pub mod apps {
    use super::PlayerSteamId;
    use crate::logger::log_function_call;
    use steamworks::AppId;

    #[napi]
    pub fn is_subscribed_app(app_id: u32) -> bool {
        let result = {
            let client = crate::client::get_client();
            client.apps().is_subscribed_app(AppId(app_id))
        };
        log_function_call("apps.is_subscribed_app", &[&app_id], &result);
        result
    }
    #[napi]
    pub fn is_app_installed(app_id: u32) -> bool {
        let result = {
            let client = crate::client::get_client();
            client.apps().is_app_installed(AppId(app_id))
        };
        log_function_call("apps.is_app_installed", &[&app_id], &result);
        result
    }

    #[napi]
    pub fn is_dlc_installed(app_id: u32) -> bool {
        let result = {
            let client = crate::client::get_client();
            client.apps().is_dlc_installed(AppId(app_id))
        };
        log_function_call("apps.is_dlc_installed", &[&app_id], &result);
        result
    }

    #[napi]
    pub fn is_subscribed_from_free_weekend() -> bool {
        let result = {
            let client = crate::client::get_client();
            client.apps().is_subscribed_from_free_weekend()
        };
        log_function_call("apps.is_subscribed_from_free_weekend", &[], &result);
        result
    }

    #[napi]
    pub fn is_vac_banned() -> bool {
        let result = {
            let client = crate::client::get_client();
            client.apps().is_vac_banned()
        };
        log_function_call("apps.is_vac_banned", &[], &result);
        result
    }

    #[napi]
    pub fn is_cybercafe() -> bool {
        let result = {
            let client = crate::client::get_client();
            client.apps().is_cybercafe()
        };
        log_function_call("apps.is_cybercafe", &[], &result);
        result
    }

    #[napi]
    pub fn is_low_violence() -> bool {
        let result = {
            let client = crate::client::get_client();
            client.apps().is_low_violence()
        };
        log_function_call("apps.is_low_violence", &[], &result);
        result
    }

    #[napi]
    pub fn is_subscribed() -> bool {
        let result = {
            let client = crate::client::get_client();
            client.apps().is_subscribed()
        };
        log_function_call("apps.is_subscribed", &[], &result);
        result
    }

    #[napi]
    pub fn app_build_id() -> i32 {
        let result = {
            let client = crate::client::get_client();
            client.apps().app_build_id()
        };
        log_function_call("apps.app_build_id", &[], &result);
        result
    }

    #[napi]
    pub fn app_install_dir(app_id: u32) -> String {
        let result = {
            let client = crate::client::get_client();
            client.apps().app_install_dir(AppId(app_id))
        };
        log_function_call("apps.app_install_dir", &[&app_id], &result);
        result
    }

    #[napi]
    pub fn app_owner() -> PlayerSteamId {
        let result = {
            let client = crate::client::get_client();
            let steam_id = client.apps().app_owner();
            PlayerSteamId::from_steamid(steam_id)
        };
        log_function_call("apps.app_owner", &[], &"PlayerSteamId");
        result
    }

    #[napi]
    pub fn available_game_languages() -> Vec<String> {
        let result = {
            let client = crate::client::get_client();
            client.apps().available_game_languages()
        };
        log_function_call("apps.available_game_languages", &[], &result);
        result
    }

    #[napi]
    pub fn current_game_language() -> String {
        let result = {
            let client = crate::client::get_client();
            client.apps().current_game_language()
        };
        log_function_call("apps.current_game_language", &[], &result);
        result
    }

    #[napi]
    pub fn current_beta_name() -> Option<String> {
        let result = {
            let client = crate::client::get_client();
            client.apps().current_beta_name()
        };
        log_function_call("apps.current_beta_name", &[], &result);
        result
    }
}
