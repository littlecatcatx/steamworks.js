use napi_derive::napi;

#[napi]
pub mod matchmaking {
    use crate::api::localplayer::PlayerSteamId;
    use crate::logger::log_function_call;
    use napi::bindgen_prelude::{BigInt, Error};
    use std::collections::HashMap;
    use steamworks::LobbyId;
    use tokio::sync::oneshot;

    #[napi]
    #[derive(Debug)]
    pub enum LobbyType {
        Private,
        FriendsOnly,
        Public,
        Invisible,
    }

    #[napi]
    pub struct Lobby {
        pub id: BigInt,
        lobby_id: LobbyId,
    }

    #[napi]
    impl Lobby {
        #[napi]
        pub async fn join(&self) -> Result<Lobby, Error> {
            let result = join_lobby(self.id.clone()).await;
            log_function_call("matchmaking.Lobby.join", &[&self.id], &result.is_ok());
            result
        }

        #[napi]
        pub fn leave(&self) {
            let client = crate::client::get_client();
            client.matchmaking().leave_lobby(self.lobby_id);
            log_function_call("matchmaking.Lobby.leave", &[&self.id], &"()");
        }

        #[napi]
        pub fn open_invite_dialog(&self) {
            let client = crate::client::get_client();
            client.friends().activate_invite_dialog(self.lobby_id);
            log_function_call("matchmaking.Lobby.open_invite_dialog", &[&self.id], &"()");
        }

        #[napi]
        pub fn get_member_count(&self) -> usize {
            let result = {
                let client = crate::client::get_client();
                client.matchmaking().lobby_member_count(self.lobby_id)
            };
            log_function_call(
                "matchmaking.Lobby.get_member_count",
                &[&self.id],
                &result,
            );
            result
        }

        #[napi]
        pub fn get_member_limit(&self) -> Option<usize> {
            let result = {
                let client = crate::client::get_client();
                client.matchmaking().lobby_member_limit(self.lobby_id)
            };
            log_function_call(
                "matchmaking.Lobby.get_member_limit",
                &[&self.id],
                &result,
            );
            result
        }

        #[napi]
        pub fn get_members(&self) -> Vec<PlayerSteamId> {
            let result = {
                let client = crate::client::get_client();
                client
                    .matchmaking()
                    .lobby_members(self.lobby_id)
                    .into_iter()
                    .map(PlayerSteamId::from_steamid)
                    .collect()
            };
            log_function_call(
                "matchmaking.Lobby.get_members",
                &[&self.id],
                &"Vec<PlayerSteamId>",
            );
            result
        }

        #[napi]
        pub fn get_owner(&self) -> PlayerSteamId {
            let result = {
                let client = crate::client::get_client();
                PlayerSteamId::from_steamid(client.matchmaking().lobby_owner(self.lobby_id))
            };
            log_function_call("matchmaking.Lobby.get_owner", &[&self.id], &result);
            result
        }

        #[napi]
        pub fn set_joinable(&self, joinable: bool) -> bool {
            let result = {
                let client = crate::client::get_client();
                client
                    .matchmaking()
                    .set_lobby_joinable(self.lobby_id, joinable)
            };
            log_function_call(
                "matchmaking.Lobby.set_joinable",
                &[&self.id, &joinable],
                &result,
            );
            result
        }

        #[napi]
        pub fn get_data(&self, key: String) -> Option<String> {
            let result = {
                let client = crate::client::get_client();
                client
                    .matchmaking()
                    .lobby_data(self.lobby_id, &key)
                    .map(|s| s.to_string())
            };
            log_function_call("matchmaking.Lobby.get_data", &[&self.id, &key], &result);
            result
        }

        #[napi]
        pub fn set_data(&self, key: String, value: String) -> bool {
            let result = {
                let client = crate::client::get_client();
                client
                    .matchmaking()
                    .set_lobby_data(self.lobby_id, &key, &value)
            };
            log_function_call(
                "matchmaking.Lobby.set_data",
                &[&self.id, &key, &value],
                &result,
            );
            result
        }

        #[napi]
        pub fn delete_data(&self, key: String) -> bool {
            let result = {
                let client = crate::client::get_client();
                client.matchmaking().delete_lobby_data(self.lobby_id, &key)
            };
            log_function_call("matchmaking.Lobby.delete_data", &[&self.id, &key], &result);
            result
        }

        /// Get an object containing all the lobby data
        #[napi]
        pub fn get_full_data(&self) -> HashMap<String, String> {
            let result = {
                let client = crate::client::get_client();

                let mut data = HashMap::new();

                let count = client.matchmaking().lobby_data_count(self.lobby_id);
                for i in 0..count {
                    let maybe_lobby_data =
                        client.matchmaking().lobby_data_by_index(self.lobby_id, i);

                    if let Some((key, value)) = maybe_lobby_data {
                        data.insert(key, value);
                    }
                }

                data
            };
            log_function_call("matchmaking.Lobby.get_full_data", &[&self.id], &result);
            result
        }

        /// Merge current lobby data with provided data in a single batch
        /// @returns true if all data was set successfully
        #[napi]
        pub fn merge_full_data(&self, data: HashMap<String, String>) -> bool {
            let result = {
                let matchmaking = crate::client::get_client().matchmaking();
                data.iter()
                    .map(|(key, value)| matchmaking.set_lobby_data(self.lobby_id, key, value))
                    .all(|x| x)
            };
            log_function_call(
                "matchmaking.Lobby.merge_full_data",
                &[&self.id, &data],
                &result,
            );
            result
        }
    }

    #[napi]
    pub async fn create_lobby(lobby_type: LobbyType, max_members: u32) -> Result<Lobby, Error> {
        let result = (|| async {
            let client = crate::client::get_client();

            let (tx, rx) = oneshot::channel();

            client.matchmaking().create_lobby(
                match lobby_type {
                    LobbyType::Private => steamworks::LobbyType::Private,
                    LobbyType::FriendsOnly => steamworks::LobbyType::FriendsOnly,
                    LobbyType::Public => steamworks::LobbyType::Public,
                    LobbyType::Invisible => steamworks::LobbyType::Invisible,
                },
                max_members,
                |result| {
                    tx.send(result).unwrap();
                },
            );

            rx.await
                .unwrap()
                .map(|lobby_id| Lobby {
                    id: BigInt::from(lobby_id.raw()),
                    lobby_id,
                })
                .map_err(|e| Error::from_reason(e.to_string()))
        })()
        .await;

        log_function_call(
            "matchmaking.create_lobby",
            &[&lobby_type, &max_members],
            &result.is_ok(),
        );
        result
    }

    #[napi]
    pub async fn join_lobby(lobby_id: BigInt) -> Result<Lobby, Error> {
        let result = (|| async {
            let client = crate::client::get_client();

            let (tx, rx) = oneshot::channel();

            client.matchmaking().join_lobby(
                steamworks::LobbyId::from_raw(lobby_id.get_u64().1),
                |result| {
                    tx.send(result).unwrap();
                },
            );

            rx.await
                .unwrap()
                .map(|lobby_id| Lobby {
                    id: BigInt::from(lobby_id.raw()),
                    lobby_id,
                })
                .map_err(|_| Error::from_reason("Failed to join lobby".to_string()))
        })()
        .await;

        log_function_call("matchmaking.join_lobby", &[&lobby_id], &result.is_ok());
        result
    }

    #[napi]
    pub async fn get_lobbies() -> Result<Vec<Lobby>, Error> {
        let result = (|| async {
            let client = crate::client::get_client();

            let (tx, rx) = oneshot::channel();

            client.matchmaking().request_lobby_list(|lobbies| {
                tx.send(lobbies).unwrap();
            });

            rx.await
                .unwrap()
                .map(|lobbies| {
                    lobbies
                        .iter()
                        .map(|lobby_id| Lobby {
                            id: BigInt::from(lobby_id.raw()),
                            lobby_id: *lobby_id,
                        })
                        .collect()
                })
                .map_err(|e| Error::from_reason(e.to_string()))
        })()
        .await;

        log_function_call("matchmaking.get_lobbies", &[], &result.is_ok());
        result
    }
}
