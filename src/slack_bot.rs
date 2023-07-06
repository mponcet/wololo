use std::sync::Arc;

use crate::db::SharedDb;
use crate::wol;
use slack_morphism::prelude::*;
pub struct SlackBot {
    db: SharedDb,
}

#[derive(Debug)]
pub enum SlackBotError {
    AppTokenMissing,
    SlackClientError(slack_morphism::errors::SlackClientError),
}

impl std::fmt::Display for SlackBotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlackBotError::AppTokenMissing => {
                write!(f, "Environment variable SLACK_APP_TOKEN missing")
            }
            SlackBotError::SlackClientError(e) => {
                write!(f, "Slack client error: {}", e)
            }
        }
    }
}

impl std::error::Error for SlackBotError {}

impl From<std::env::VarError> for SlackBotError {
    fn from(_: std::env::VarError) -> Self {
        SlackBotError::AppTokenMissing
    }
}

impl From<slack_morphism::errors::SlackClientError> for SlackBotError {
    fn from(e: slack_morphism::errors::SlackClientError) -> Self {
        SlackBotError::SlackClientError(e)
    }
}

struct UserState {
    db: SharedDb,
}

impl SlackBot {
    pub fn new(db: SharedDb) -> Self {
        Self { db }
    }

    async fn command_events(
        event: SlackCommandEvent,
        _client: Arc<SlackHyperClient>,
        user_state: SlackClientEventsUserState,
    ) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
        println!("{:#?}", event);
        let slack_user_id = event.user_id;
        let storage = user_state.read().await;
        let db = &storage.get_user_state::<UserState>().unwrap().db;

        if let Some(mac) = db.get_mac_by_slack_user_id(&slack_user_id.0) {
            wol::send_wol(mac);
            Ok(SlackCommandEventResponse::new(
                SlackMessageContent::new().with_text(format!("wololo {}", mac)),
            ))
        } else {
            Ok(SlackCommandEventResponse::new(
                SlackMessageContent::new().with_text("nope".to_string()),
            ))
        }
    }

    pub fn start(&self) -> Result<(), SlackBotError> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let app_token_value: SlackApiTokenValue = std::env::var("SLACK_APP_TOKEN")?.into();
        let app_token = SlackApiToken::new(app_token_value);

        runtime.block_on(async {
            let client = SlackClient::new(SlackClientHyperConnector::new());

            let socket_mode_callbacks = SlackSocketModeListenerCallbacks::new()
                .with_command_events(SlackBot::command_events);

            let listener_environment = SlackClientEventsListenerEnvironment::new(Arc::new(client))
                .with_user_state(UserState {
                    db: self.db.clone(),
                });

            let socket_mode_listener = SlackClientSocketModeListener::new(
                &SlackClientSocketModeConfig::new(),
                Arc::new(listener_environment),
                socket_mode_callbacks,
            );

            socket_mode_listener.listen_for(&app_token).await?;

            socket_mode_listener.serve().await;

            Ok(())
        })
    }

    fn stop(&self) {
        todo!()
    }
}
