use core::fmt;
use slack_morphism::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;

use crate::db::SharedDb;
use crate::wol;

const RETRY_TCP_CHECK: u8 = 6;

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

enum BotResponse {
    MagicPacketSent,
    MagicPacketError,
    UserNotFound,
    HostUp,
    HostDown,
}

impl std::fmt::Display for BotResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BotResponse::MagicPacketSent => "Magic packet sent.",
                BotResponse::MagicPacketError => "Error while sending magic packet.",
                BotResponse::UserNotFound => "User not found.",
                BotResponse::HostUp => "Host is up !",
                BotResponse::HostDown => "Host is down :'(",
            }
        )
    }
}

impl SlackBot {
    pub fn new(db: SharedDb) -> Self {
        Self { db }
    }

    async fn command_events(
        event: SlackCommandEvent,
        client: Arc<SlackHyperClient>,
        user_state: SlackClientEventsUserState,
    ) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
        let slack_user_id = event.user_id.to_string();
        let storage = user_state.read().await;
        let db = &storage.get_user_state::<UserState>().unwrap().db;

        let device = db.get_device_by_slack_user_id(&slack_user_id);
        let bot_answer = match device {
            Some(device) => match wol::send_wol(&device.mac).await {
                Ok(_) => {
                    if let Some(tcp_check_addr) = device.tcp_check_addr.clone() {
                        SlackBot::post_wol_check(event, client, tcp_check_addr).await;
                    }

                    BotResponse::MagicPacketSent
                }
                Err(_) => BotResponse::MagicPacketError,
            },
            None => BotResponse::UserNotFound,
        };

        Ok(SlackCommandEventResponse::new(
            SlackMessageContent::new().with_text(bot_answer.to_string()),
        ))
    }

    async fn post_wol_check(
        event: SlackCommandEvent,
        client: Arc<SlackHyperClient>,
        tcp_check_addr: String,
    ) {
        tokio::spawn(async move {
            let mut retries = 0;
            while retries < RETRY_TCP_CHECK {
                // wait for computer to start
                tokio::time::sleep(Duration::from_secs(5)).await;

                if TcpStream::connect(&tcp_check_addr).await.is_ok() {
                    let _ = client
                        .respond_to_event(
                            &event.response_url,
                            &SlackApiPostWebhookMessageRequest::new(
                                SlackMessageContent::new()
                                    .with_text(BotResponse::HostUp.to_string()),
                            ),
                        )
                        .await;
                    break;
                }

                retries += 1;
            }

            if retries == RETRY_TCP_CHECK {
                let _ = client
                    .respond_to_event(
                        &event.response_url,
                        &SlackApiPostWebhookMessageRequest::new(
                            SlackMessageContent::new().with_text(BotResponse::HostDown.to_string()),
                        ),
                    )
                    .await;
            }
        });
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
