use std::sync::{Arc, Mutex};
use std::time::Duration;
use azalea::{swarm::prelude::*, protocol::ServerAddress, ClientInformation, prelude::*, DefaultPlugins, DefaultBotPlugins};
use azalea::protocol::packets::configuration::serverbound_client_information_packet::{ChatVisibility, HumanoidArm, ModelCustomization};
use azalea::swarm::DefaultSwarmPlugins;

pub(crate) async fn main() {
    let account = Account::microsoft(std::env::var("MINECRAFT_MAIL").unwrap().as_str())
        .await
        .unwrap();
    let address = ServerAddress::try_from("mc.hypixel.net:25565").unwrap();

    let swarm_builder = SwarmBuilder::new_without_plugins()
        .add_plugins((
            DefaultPlugins,
            DefaultBotPlugins,
            DefaultSwarmPlugins
        ))
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .set_swarm_state(SwarmState)
        .add_account_with_state(
            account,
            State {
                ..Default::default()
            },
        );
    if let Err(why) = swarm_builder.start(address).await {
        println!("minecraft error{:?}", why)
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BotTask {
    #[default]
    None,
}

#[derive(Component, Clone)]
pub struct State {
    pub task: Arc<Mutex<BotTask>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            task: Arc::new(Mutex::new(BotTask::None)),
        }
    }
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Init => {
            bot.set_client_information(ClientInformation {
                language: "en_US".to_string(),
                view_distance: 2,
                chat_visibility: ChatVisibility::Full,
                chat_colors: true,
                model_customization: ModelCustomization {
                    cape: true,
                    jacket: true,
                    left_sleeve: true,
                    right_sleeve: true,
                    left_pants: true,
                    right_pants: true,
                    hat: true,
                },
                main_hand: HumanoidArm::Right,
                text_filtering_enabled: false,
                allows_listing: false,
            }).await?
        },
        Event::Login => {
            println!("Logged in as {}", bot.profile.name.clone());
        },
        _ => {
        }
    };
    match state {
        _ => {}
    };
    Ok(())
}

#[derive(Resource, Default, Clone)]
struct SwarmState;

async fn swarm_handle(
    mut swarm: Swarm,
    event: SwarmEvent,
    _state: SwarmState,
) -> anyhow::Result<()> {
    match &event {
        SwarmEvent::Disconnect(account) => {
            println!("bot got kicked! {}", account.username);
            tokio::time::sleep(Duration::from_secs(5)).await;
            swarm.add_and_retry_forever(account, State::default()).await;
        },
        SwarmEvent::Chat(chat) => println!("{}", chat.message().to_ansi()),
        SwarmEvent::Init => {
            println!("Swarm initialised")
        },
            _ => {}
    }

    Ok(())
}