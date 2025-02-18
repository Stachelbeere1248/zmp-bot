use crate::Error;
use getset::Getters;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Getters)]
pub(crate) struct Response {
    pub(crate) bots: Vec<Bot>,
}

#[derive(Deserialize, Getters)]
#[getset(get = "pub(crate)")]
pub(crate) struct Bot {
    username: String,
    list_type: ListType,
    list: Vec<String>,
    strict: bool,
    /* we don't care abt lobby data
     * lobby_name: String,
     * lobby_number: u8,
     */
    in_party: bool,
    /* we don't care what script the bot is running
     * client_gui_version: String,
     * client_version: String,
     */
    last_updated: f64,
    last_updated_utc: String, //TODO: DateTime
    note: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ListType {
    Whitelist,
    Blacklist,
}

pub(crate) async fn fetch_all(client: &Client) -> Result<Response, Error> {
    let url = "https://localhost:6969/list";
    let response: Response = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(response)
}
