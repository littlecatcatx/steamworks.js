use napi_derive::napi;

#[napi]
pub mod cloud {
    use crate::logger::log_function_call;
    use napi::bindgen_prelude::{BigInt, Error};
    use std::io::Read;
    use std::io::Write;

    #[napi]
    pub struct FileInfo {
        pub name: String,
        pub size: BigInt,
    }

    #[napi]
    pub fn is_enabled_for_account() -> bool {
        let result = {
            let client = crate::client::get_client();
            client.remote_storage().is_cloud_enabled_for_account()
        };
        log_function_call("cloud.is_enabled_for_account", &[], &result);
        result
    }

    #[napi]
    pub fn is_enabled_for_app() -> bool {
        let result = {
            let client = crate::client::get_client();
            client.remote_storage().is_cloud_enabled_for_app()
        };
        log_function_call("cloud.is_enabled_for_app", &[], &result);
        result
    }

    #[napi]
    pub fn set_enabled_for_app(enabled: bool) {
        let client = crate::client::get_client();
        client.remote_storage().set_cloud_enabled_for_app(enabled);
        log_function_call("cloud.set_enabled_for_app", &[&enabled], &"()");
    }

    #[napi]
    pub fn read_file(name: String) -> Result<String, Error> {
        let result = (|| {
            let client = crate::client::get_client();
            let mut buf: String = String::new();
            let size = client
                .remote_storage()
                .file(&name)
                .read()
                .read_to_string(&mut buf);

            match size {
                Ok(_) => Ok(buf),
                Err(e) => Err(Error::from_reason(format!("Failed to read file: {}", e))),
            }
        })();
        log_function_call("cloud.read_file", &[&name], &result.is_ok());
        result
    }

    #[napi]
    pub fn write_file(name: String, content: String) -> bool {
        let result = {
            let client = crate::client::get_client();
            let file = client.remote_storage().file(&name);

            let buf = content.as_bytes();
            file.write().write_all(buf).is_ok()
        };
        log_function_call("cloud.write_file", &[&name, &content], &result);
        result
    }

    #[napi]
    pub fn delete_file(name: String) -> bool {
        let result = {
            let client = crate::client::get_client();
            let file = client.remote_storage().file(&name);

            file.delete()
        };
        log_function_call("cloud.delete_file", &[&name], &result);
        result
    }

    #[napi]
    pub fn file_exists(name: String) -> bool {
        let result = {
            let client = crate::client::get_client();
            let file = client.remote_storage().file(&name);

            file.exists()
        };
        log_function_call("cloud.file_exists", &[&name], &result);
        result
    }

    #[napi]
    pub fn list_files() -> Vec<FileInfo> {
        let result = {
            let client = crate::client::get_client();
            client
                .remote_storage()
                .files()
                .into_iter()
                .map(|identity| FileInfo {
                    name: identity.name,
                    size: BigInt::from(identity.size),
                })
                .collect()
        };
        log_function_call("cloud.list_files", &[], &"Vec<FileInfo>");
        result
    }
}
