use crate::{
    device::{Device, DeviceName, MacAddress},
    repository::SharedDeviceRepository,
    service::WakeOnLanService,
    wol,
};

use slack_morphism::prelude::*;
use std::sync::Arc;
use tokio;

const WOLOLO_COMMANND: &str = "/wololo";
const WOLOLO_HELP: &str = "help:
 /wololo add NAME MAC
 /wololo del NAME
 /wololo wake NAME|MAC";

struct UserState {
    repo: SharedDeviceRepository,
}

pub struct SlackWolService;

impl SlackWolService {
    pub fn new() -> Self {
        Self
    }

    async fn command_events(
        event: SlackCommandEvent,
        _client: Arc<SlackHyperClient>,
        user_state: SlackClientEventsUserState,
    ) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
        println!("{:#?}", event);

        if event.command.as_ref() != WOLOLO_COMMANND {
            return Err("Unexpected command".into());
        }

        if let Some(text) = event.text {
            let storage = user_state.read().await;
            let repo = &storage.get_user_state::<UserState>().unwrap().repo;
            let cmds: Vec<_> = text.split_whitespace().collect();

            let message = match (cmds.first(), cmds.get(1), cmds.get(2)) {
                (Some(&"add"), Some(&name), Some(&mac)) => match Device::try_from((name, mac)) {
                    Ok(device) => match repo.insert(device) {
                        Ok(_) => format!("Device {} added", name),
                        Err(e) => format!("Couldn't add device {} ({})", name, e),
                    },
                    Err(e) => format!("Failed to parse device name and/or mac address ({})", e),
                },
                (Some(&"del"), Some(&name), None) => DeviceName::try_from(name).map_or_else(
                    |e| e,
                    |ref name| match repo.delete(name) {
                        Ok(_) => format!("Device {} deleted", name),
                        Err(e) => format!("Couldn't delete device {} ({})", name, e),
                    },
                ),
                (Some(&"wake"), Some(&name_or_mac), None) => {
                    if let Ok(ref mac) = MacAddress::try_from(name_or_mac) {
                        wol::wake(mac).map_or_else(
                            |e| format!("Couldn't send magic packet ({})", e),
                            |_| format!("Magic packet sent to {}", mac),
                        )
                    } else if let Ok(ref name) = DeviceName::try_from(name_or_mac) {
                        match repo.fetch_by_name(name) {
                            Some(device) => wol::wake(&device.mac).map_or_else(
                                |e| format!("Couldn't send magic packet ({})", e),
                                |_| format!("Magic packet sent to {}", device.mac),
                            ),
                            None => format!("Could't find device {}", name_or_mac),
                        }
                    } else {
                        format!("{} is not a valid name or mac address", name_or_mac)
                    }
                }
                _ => WOLOLO_HELP.to_owned(),
            };

            Ok(SlackCommandEventResponse::new(
                SlackMessageContent::new().with_text(message),
            ))
        } else {
            Ok(SlackCommandEventResponse::new(
                SlackMessageContent::new().with_text(WOLOLO_HELP.into()),
            ))
        }
    }
}

impl WakeOnLanService for SlackWolService {
    fn run(
        &self,
        repo: SharedDeviceRepository,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Bridge sync and async code here
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

            let socket_mode_callbacks = SlackSocketModeListenerCallbacks::new()
                .with_command_events(SlackWolService::command_events);

            let listener_environment = Arc::new(
                SlackClientEventsListenerEnvironment::new(client.clone())
                    .with_user_state(UserState { repo }),
            );

            let socket_mode_listener = SlackClientSocketModeListener::new(
                &SlackClientSocketModeConfig::new(),
                listener_environment,
                socket_mode_callbacks,
            );

            let app_token_value: SlackApiTokenValue =
                std::env::var("SLACK_APP_TOKEN").unwrap().into();
            let app_token: SlackApiToken = SlackApiToken::new(app_token_value);

            socket_mode_listener.listen_for(&app_token).await?;

            socket_mode_listener.serve().await;

            Ok(())
        })
    }
}
