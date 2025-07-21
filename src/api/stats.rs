use napi_derive::napi;

#[napi]
pub mod stats {
    use crate::logger::log_function_call;

    #[napi]
    pub fn get_int(name: String) -> Option<i32> {
        let result = {
            let client = crate::client::get_client();
            client.user_stats().get_stat_i32(&name).ok()
        };
        log_function_call("stats.get_int", &[&name], &result);
        result
    }

    #[napi]
    pub fn set_int(name: String, value: i32) -> bool {
        let result = {
            let client = crate::client::get_client();
            client.user_stats().set_stat_i32(&name, value).is_ok()
        };
        log_function_call("stats.set_int", &[&name, &value], &result);
        result
    }

    #[napi]
    pub fn store() -> bool {
        let result = {
            let client = crate::client::get_client();
            client.user_stats().store_stats().is_ok()
        };
        log_function_call("stats.store", &[], &result);
        result
    }

    #[napi]
    pub fn reset_all(achievements_too: bool) -> bool {
        let result = {
            let client = crate::client::get_client();
            client
                .user_stats()
                .reset_all_stats(achievements_too)
                .is_ok()
        };
        log_function_call("stats.reset_all", &[&achievements_too], &result);
        result
    }
}
