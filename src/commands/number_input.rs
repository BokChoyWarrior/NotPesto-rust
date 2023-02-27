use serenity::builder;
use serenity::model::application::interaction::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use tracing::debug;

pub fn run(options: &[CommandDataOption]) -> String {
    let option1 = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");
    let option2 = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    debug!("{option1:?} {option2:?}");

    5.to_string()

    // if let CommandDataOptionValue::User(user, _member) = option {
    //     format!("{}'s id is {}", user.tag(), user.id)
    // } else {
    //     "Please provide a valid user".to_string()
    // }
}
pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("numberinput")
        .description("Test command for number input")
        .create_option(|option| {
            option
                .name("int")
                .description("An integer from 5 to 10")
                .kind(CommandOptionType::Integer)
                .min_int_value(5)
                .max_int_value(10)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("number")
                .description("A float from -3.3 to 234.5")
                .kind(CommandOptionType::Number)
                .min_number_value(-3.3)
                .max_number_value(234.5)
                .required(true)
        })
}
