#[derive(Debug)]
pub enum Error {
    SqlxError(sqlx::Error),
    HttpsError(reqwest::Error),
    ParsingError(serde_json::Error),
    SerenityError(serenity::Error),
    OnCooldown(std::time::Duration),
    Other(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::SqlxError(e) => write!(f, "SQLx Error: {}", e),
            Error::HttpsError(e) => write!(f, "HTTPS Error (Hypixel / Mojang API):\n{}", e),
            Error::ParsingError(e) => write!(f, "Parsing Error:\n {}", e),
            Error::SerenityError(e) => write!(f, "Serenity Error:\n {}", e),
            Error::OnCooldown(d) => write!(f, "This command is on cooldown. {}s remaining.", d.as_secs()),
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