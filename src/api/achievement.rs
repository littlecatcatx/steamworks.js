use napi_derive::napi;

use crate::logger::log_function_call;

#[napi]
pub mod achievement {
    use super::log_function_call;

    #[napi]
    pub fn activate(achievement: String) -> bool {
        let result = {
            let client = crate::client::get_client();
            client
                .user_stats()
                .achievement(&achievement)
                .set()
                .and_then(|_| client.user_stats().store_stats())
                .is_ok()
        };
        log_function_call("achievement.activate", &[&achievement], &result);
        result
    }

    #[napi]
    pub fn is_activated(achievement: String) -> bool {
        let result = {
            let client = crate::client::get_client();
            client
                .user_stats()
                .achievement(&achievement)
                .get()
                .unwrap_or(false)
        };
        log_function_call("achievement.is_activated", &[&achievement], &result);
        result
    }

    #[napi]
    pub fn clear(achievement: String) -> bool {
        let result = {
            let client = crate::client::get_client();
            client
                .user_stats()
                .achievement(&achievement)
                .clear()
                .and_then(|_| client.user_stats().store_stats())
                .is_ok()
        };
        log_function_call("achievement.clear", &[&achievement], &result);
        result
    }

    #[napi]
    pub fn names() -> Vec<String> {
        let result = {
            let client = crate::client::get_client();
            client
                .user_stats()
                .get_achievement_names()
                .expect("Failed to get achievement names")
        };
        log_function_call("achievement.names", &[], &result);
        result
    }
}
