use crate::{
    command::{
        CommandError, CommandExecutor, CommandSender,
        args::{Arg, ConsumedArgs, simple::SimpleArgConsumer},
        tree::CommandTree,
        tree::builder::argument,
    },
    data::{banned_ip_data::BANNED_IP_LIST, banned_player_data::BANNED_PLAYER_LIST},
};
use CommandError::InvalidConsumption;
use async_trait::async_trait;
use pumpkin_util::text::TextComponent;

const NAMES: [&str; 1] = ["banlist"];
const DESCRIPTION: &str = "shows the banlist";

const ARG_LIST_TYPE: &str = "ips|players";

struct ListExecutor;

#[async_trait]
impl CommandExecutor for ListExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Simple(list_type)) = args.get(&ARG_LIST_TYPE) else {
            return Err(InvalidConsumption(Some(ARG_LIST_TYPE.into())));
        };

        match *list_type {
            "ips" => {
                let lock = &BANNED_IP_LIST.read().await;
                let entries = lock
                    .banned_ips
                    .iter()
                    .map(|entry| {
                        (
                            entry.ip.to_string(),
                            entry.source.clone(),
                            entry.reason.clone(),
                        )
                    })
                    .collect();

                handle_banlist(entries, sender).await;
            }
            "players" => {
                let lock = &BANNED_PLAYER_LIST.read().await;
                let entries = lock
                    .banned_players
                    .iter()
                    .map(|entry| {
                        (
                            entry.name.clone(),
                            entry.source.clone(),
                            entry.reason.clone(),
                        )
                    })
                    .collect();

                handle_banlist(entries, sender).await;
            }
            _ => {
                return Err(CommandError::CommandFailed(Box::new(TextComponent::text(
                    "Incorrect argument for command".to_string(),
                ))));
            }
        }

        Ok(())
    }
}

struct ListAllExecutor;

#[async_trait]
impl CommandExecutor for ListAllExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &crate::server::Server,
        _args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let mut entries = Vec::new();
        for entry in &BANNED_PLAYER_LIST.read().await.banned_players {
            entries.push((
                entry.name.clone(),
                entry.source.clone(),
                entry.reason.clone(),
            ));
        }

        for entry in &BANNED_IP_LIST.read().await.banned_ips {
            entries.push((
                entry.ip.to_string(),
                entry.source.clone(),
                entry.reason.clone(),
            ));
        }

        handle_banlist(entries, sender).await;
        Ok(())
    }
}

/// `Vec<(name, source, reason)>`
async fn handle_banlist(list: Vec<(String, String, String)>, sender: &CommandSender) {
    if list.is_empty() {
        sender
            .send_message(TextComponent::translate("commands.banlist.none", []))
            .await;
        return;
    }

    sender
        .send_message(TextComponent::translate(
            "commands.banlist.list",
            [TextComponent::text(list.len().to_string())],
        ))
        .await;

    for (name, source, reason) in list {
        sender
            .send_message(TextComponent::translate(
                "commands.banlist.entry",
                [
                    TextComponent::text(name),
                    TextComponent::text(source),
                    TextComponent::text(reason),
                ],
            ))
            .await;
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .execute(ListAllExecutor)
        .then(argument(ARG_LIST_TYPE, SimpleArgConsumer).execute(ListExecutor))
}
