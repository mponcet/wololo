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
                write!(f, "Error: environment variable SLACK_APP_TOKEN missing")
            }
            SlackBotError::SlackClientError(e) => {
                write!(f, "Error: {}", e)
            }
        }
    }
}

impl std::error::Error for SlackBotError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SlackBotError::AppTokenMissing => None,
            SlackBotError::SlackClientError(ref e) => Some(e),
        }
    }
}

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

        let bot_answer = match db.get_mac_by_slack_user_id(&slack_user_id.0) {
            Some(mac) => {
                if wol::send_wol(mac).await.is_ok() {
                    format!("Magic packet sent to {mac}")
                } else {
                    "Error while sending magic packet".to_string()
                }
            }
            None => "User not found".to_string(),
        };

        Ok(SlackCommandEventResponse::new(
            SlackMessageContent::new().with_text(bot_answer),
        ))
    }

    pub async fn start(&self) -> Result<(), SlackBotError> {
        let app_token_value: SlackApiTokenValue = std::env::var("SLACK_APP_TOKEN")?.into();
        let app_token = SlackApiToken::new(app_token_value);

        let client = SlackClient::new(SlackClientHyperConnector::new());

        let socket_mode_callbacks =
            SlackSocketModeListenerCallbacks::new().with_command_events(SlackBot::command_events);

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
    }
}
