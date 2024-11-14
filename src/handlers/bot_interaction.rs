use serenity::all::{ButtonStyle, ComponentInteraction};
use serenity::all::ButtonStyle::Success;
use serenity::all::ComponentInteractionDataKind;
use serenity::all::Context;
use serenity::all::CreateActionRow;
use serenity::all::CreateButton;
use serenity::all::CreateInteractionResponse::Message;
use serenity::all::CreateInteractionResponseMessage;
use serenity::all::CreateMessage;
use serenity::all::EditMessage;
use serenity::all::GuildId;
use serenity::all::Interaction;
use serenity::all::ReactionType;
use serenity::all::RoleId;

use crate::Data;
use crate::error::Error;

pub(crate) async fn component(ctx: &Context, interaction: &Interaction, data: &Data) -> Result<(), Error> {
    let component = interaction.clone().message_component().unwrap();
    match component.data.kind {
        ComponentInteractionDataKind::Button => button(ctx, component, data).await,
        _ => Ok(()),
    }
}

async fn button(ctx: &Context, mut interaction: ComponentInteraction, data: &Data) -> Result<(), Error> {
    let m = &interaction.message;
    let u = m.mentions.first().expect("Message did not mention a user.");
    match interaction.data.custom_id.as_str() {
        "accept_verification" => {
            let _dm = u
                .direct_message(ctx, CreateMessage::new().content("Your verified minecraft account was approved."))
                .await?;
            let member = m
                .guild_id
                .unwrap_or(GuildId::new(1256217633959841853_u64))
                .member(ctx, u.id)
                .await?;
            member.add_role(ctx, RoleId::new(1256218805911425066_u64)).await?;
            member.remove_role(ctx, RoleId::new(1256253358701023232_u64)).await?;
            interaction
                .message
                .edit(
                    ctx,
                    EditMessage::new().components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new("accept_verification")
                            .emoji(ReactionType::from('✅'))
                            .style(Success)
                            .disabled(true),
                        CreateButton::new("deny_verification")
                            .emoji(ReactionType::from('❌'))
                            .style(ButtonStyle::Secondary)
                            .disabled(true),
                        CreateButton::new("list_accounts").emoji(ReactionType::from('📜')),
                    ])]),
                )
                .await?;
            Ok(())
        }
        "deny_verification" => {
            let _dm = u
                .direct_message(ctx, CreateMessage::new().content("Your verified minecraft account was denied."))
                .await?;
            interaction
                .message
                .edit(
                    ctx,
                    EditMessage::new().components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new("accept_verification")
                            .emoji(ReactionType::from('✅'))
                            .style(ButtonStyle::Secondary)
                            .disabled(true),
                        CreateButton::new("deny_verification")
                            .emoji(ReactionType::from('❌'))
                            .style(ButtonStyle::Danger)
                            .disabled(true),
                        CreateButton::new("list_accounts").emoji(ReactionType::from('📜')),
                    ])]),
                )
                .await?;

            Ok(())
        }
        "list_accounts" => {
            let user = interaction.message.mentions.first().unwrap();
            let s: String = crate::commands::accountv2::list_string(&data.sqlite_pool, user).await?;
            interaction
                .create_response(ctx, Message(CreateInteractionResponseMessage::new().content(s).ephemeral(true)))
                .await?;
            Ok(())
        }
        _ => Ok(()),
    }
}
