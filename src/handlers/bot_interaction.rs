use serenity::all::{ComponentInteraction, ComponentInteractionDataKind, Context, CreateMessage, EditMessage, GuildId, Interaction, RoleId};

use crate::Error;

pub(crate) async fn component(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let component = interaction.clone().message_component().unwrap();
    match component.data.kind {
        ComponentInteractionDataKind::Button => button(ctx, component).await,
        _ => Ok(())
    }
}

async fn button(ctx: &Context, mut component: ComponentInteraction) -> Result<(), Error> {
    let m = component.message.clone();
    let u = m.mentions.first().expect("Message did not mention a user.");
    match component.data.custom_id.as_str() {
        "accept_verification" => {
            let _dm = u.direct_message(ctx, CreateMessage::new()
                .content("Your verified minecraft account was approved.")).await?;
            let member = m.guild_id.unwrap_or(GuildId::new(1256217633959841853_u64)).member(ctx, u.id).await?;
            member.add_role(ctx, RoleId::new(1256218805911425066_u64)).await?;
            member.remove_role(ctx, RoleId::new(1256253358701023232_u64)).await?;
            component.message.edit(ctx, EditMessage::new().components(vec![])).await?;
            Ok(())
        }
        "deny_verification" => {
            let _dm = u.direct_message(ctx, CreateMessage::new()
                .content("Your verified minecraft account was denied.")).await?;
            component.message.edit(ctx, EditMessage::new().components(vec![])).await?;
            Ok(())
        }
        _ => Ok(())
    }
}