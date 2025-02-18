use crate::Caches;
use crate::Error;
use reqwest::Client;
use std::time::Duration;
use std::time::Instant;

const TTL: Duration = Duration::from_days(1);

#[derive(serde::Deserialize)]
struct Response {
    #[serde(rename = "id")]
    uuid: String,
    name: String,
}

// :trollface:
macro_rules! cache_hit_handler {
    ($name:ident, $url:expr) => {
        macro_rules! inner1 {
            ($c:expr, $input:expr) => {
                $c.$name.get($input).and_then(|a| {
                    if a.1 > Instant::now() + TTL {
                        None
                    } else {
                        Some(a.0.clone())
                    }
                })
            };
        }

        macro_rules! inner2 {
            ($cli:expr, $input:expr, $c:expr) => {{
                let a = $cli
                    .get(format!($url, $input.as_str()))
                    .send()
                    .await?
                    .error_for_status()?
                    .json::<Response>()
                    .await?
                    .$name;
                let _old = $c.$name.insert($input, (a.clone(), Instant::now()));
                a
            }};
        }

        pub(crate) async fn $name<'a>(
            c: &'a Caches,
            cli: &'a Client,
            input: String,
        ) -> Result<String, Error> {
            match inner1!(c, &input) {
                None => {
                    let updated = inner2!(cli, input, c);
                    Ok(updated)
                }
                Some(hit) => Ok(hit),
            }
        }
    };
}

cache_hit_handler!(
    name,
    "https://api.minecraftservices.com/minecraft/profile/lookup/{}"
);

cache_hit_handler!(uuid, "https://api.mojang.com/users/profiles/minecraft/{}");
