use napi_derive::napi;

#[napi]
pub mod utils {
    use crate::logger::log_function_call;
    use steamworks::FloatingGamepadTextInputMode as kFloatingGamepadTextInputMode;
    use steamworks::GamepadTextInputLineMode as kGamepadTextInputLineMode;
    use steamworks::GamepadTextInputMode as kGamepadTextInputMode;
    use tokio::sync::oneshot;

    #[napi]
    pub fn get_app_id() -> u32 {
        let result = {
            let client = crate::client::get_client();
            client.utils().app_id().0
        };
        log_function_call("utils.get_app_id", &[], &result);
        result
    }

    #[napi]
    pub fn get_server_real_time() -> u32 {
        let result = {
            let client = crate::client::get_client();
            client.utils().get_server_real_time()
        };
        log_function_call("utils.get_server_real_time", &[], &result);
        result
    }

    #[napi]
    pub fn is_steam_running_on_steam_deck() -> bool {
        let result = {
            let client = crate::client::get_client();
            client.utils().is_steam_running_on_steam_deck()
        };
        log_function_call("utils.is_steam_running_on_steam_deck", &[], &result);
        result
    }

    #[napi]
    #[derive(Debug)]
    pub enum GamepadTextInputMode {
        Normal,
        Password,
    }

    #[napi]
    #[derive(Debug)]
    pub enum GamepadTextInputLineMode {
        SingleLine,
        MultipleLines,
    }

    /// @returns the entered text, or null if cancelled or could not show the input
    #[napi]
    pub async fn show_gamepad_text_input(
        input_mode: GamepadTextInputMode,
        input_line_mode: GamepadTextInputLineMode,
        description: String,
        max_characters: u32,
        existing_text: Option<String>,
    ) -> Option<String> {
        let result = (|| async {
            let client = crate::client::get_client();

            let (tx, rx) = oneshot::channel();
            let mut tx = Some(tx);

            let opened = client.utils().show_gamepad_text_input(
                match input_mode {
                    GamepadTextInputMode::Normal => kGamepadTextInputMode::Normal,
                    GamepadTextInputMode::Password => kGamepadTextInputMode::Password,
                },
                match input_line_mode {
                    GamepadTextInputLineMode::SingleLine => kGamepadTextInputLineMode::SingleLine,
                    GamepadTextInputLineMode::MultipleLines => {
                        kGamepadTextInputLineMode::MultipleLines
                    }
                },
                &description,
                max_characters,
                existing_text.as_deref(),
                move |dismissed_data| {
                    if let Some(tx) = tx.take() {
                        let text = client
                            .utils()
                            .get_entered_gamepad_text_input(&dismissed_data);
                        tx.send(text).unwrap();
                    }
                },
            );

            if opened {
                rx.await.unwrap()
            } else {
                None
            }
        })()
        .await;

        log_function_call(
            "utils.show_gamepad_text_input",
            &[
                &input_mode,
                &input_line_mode,
                &description,
                &max_characters,
                &existing_text,
            ],
            &result,
        );
        result
    }

    #[napi]
    #[derive(Debug)]
    pub enum FloatingGamepadTextInputMode {
        SingleLine,
        MultipleLines,
        Email,
        Numeric,
    }

    /// @returns true if the floating keyboard was shown, otherwise, false
    #[napi]
    pub async fn show_floating_gamepad_text_input(
        keyboard_mode: FloatingGamepadTextInputMode,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> bool {
        let result = (|| async {
            let client = crate::client::get_client();

            let (tx, rx) = oneshot::channel();
            let mut tx = Some(tx);

            let opened = client.utils().show_floating_gamepad_text_input(
                match keyboard_mode {
                    FloatingGamepadTextInputMode::SingleLine => {
                        kFloatingGamepadTextInputMode::SingleLine
                    }
                    FloatingGamepadTextInputMode::MultipleLines => {
                        kFloatingGamepadTextInputMode::MultipleLines
                    }
                    FloatingGamepadTextInputMode::Email => kFloatingGamepadTextInputMode::Email,
                    FloatingGamepadTextInputMode::Numeric => kFloatingGamepadTextInputMode::Numeric,
                },
                x,
                y,
                width,
                height,
                move || {
                    if let Some(tx) = tx.take() {
                        tx.send(true).unwrap();
                    }
                },
            );

            if opened {
                rx.await.unwrap()
            } else {
                false
            }
        })()
        .await;

        log_function_call(
            "utils.show_floating_gamepad_text_input",
            &[&keyboard_mode, &x, &y, &width, &height],
            &result,
        );
        result
    }
}
