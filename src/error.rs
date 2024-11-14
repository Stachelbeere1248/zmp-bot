use poise::{CreateReply, FrameworkError};

use crate::Data;

macro_rules! reply_fail_handler {
    ($fut:expr) => {{
        if let Err(e) = $fut.await {
            tracing::error!("Fatal error while sending error message: {}", e);
        }
    }};
}

#[derive(Debug)]
pub enum Error {
    SqlxError(sqlx::Error),
    HttpsError(reqwest::Error),
    ParsingError(serde_json::Error),
    SerenityError(serenity::Error),
    OnCooldown(std::time::Duration),
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::SqlxError(e) => write!(f, "SQLx Error: {}", e),
            Error::HttpsError(e) => write!(f, "HTTPS Error (Hypixel / Mojang API):\n{}", e),
            Error::ParsingError(e) => write!(f, "Parsing Error:\n {}", e),
            Error::SerenityError(e) => write!(f, "Discord Error:\n {}", e),
            Error::OnCooldown(d) => {
                write!(f, "This command is on cooldown. {}s remaining.", d.as_secs())
            }
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Error::SqlxError(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::HttpsError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::ParsingError(error)
    }
}

impl From<serenity::Error> for Error {
    fn from(error: serenity::Error) -> Self {
        Error::SerenityError(error)
    }
}

pub(crate) async fn handle_error<'a>(error: FrameworkError<'a, Data, Error>) {
    match error {
        FrameworkError::Command { error, ctx, .. } => {
            reply_fail_handler!(ctx.send(CreateReply::default().content(error.to_string()).ephemeral(true)))
        },
        FrameworkError::CommandStructureMismatch { description, ctx, .. } => {
            reply_fail_handler!(ctx.send(
                CreateReply::default()
                    .content(format!(
                        "# Command arguments did not match. The command probably has been updated recently. Try reloading Discord. \
                         Description:\n{}",
                        description
                    ))
                    .ephemeral(true)
            ))
        }
        other => reply_fail_handler!(poise::builtins::on_error(other)),
    }
}
