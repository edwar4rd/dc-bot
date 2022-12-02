use dc_bot::prelude::*;

async fn handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::InteractionCreate { interaction } => match interaction.kind() {
            serenity::InteractionType::MessageComponent => {
                let interaction = interaction.clone().message_component().unwrap();
                match interaction.data.component_type {
                    serenity::ComponentType::Button => {
                        if interaction.data.custom_id == "test_auth.auth_btn" {
                            commands::test_auth_btn_handler(
                                ctx,
                                &ResponsibleInteraction::MessageComponent(&interaction),
                            )
                            .await?
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}

/// Show a help menu
#[poise::command(slash_command, prefix_command)]
async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration::default(),
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::say(),
                commands::早安(),
                commands::ping(),
                commands::neofetch(),
                commands::fumo(),
                commands::botinfo(),
                commands::test_input(),
                commands::test_auth(),
                help(),
            ],
            event_handler: |ctx, event, _framework, data| Box::pin(handler(ctx, event, data)),
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
