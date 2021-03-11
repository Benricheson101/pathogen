use lazy_static::lazy_static;
use regex::Regex;

const MIN_SNOWFLAKE_LEN: u8 = 17;
const MAX_SNOWFLAKE_LEN: u8 = 18;

const MIN_EMOJI_LEN: u8 = 2;
const MAX_EMOJI_LEN: u8 = 32;

const MIN_COMNMAD_LEN: u8 = 1;
const MAX_COMMAND_LEN: u8 = 32;

lazy_static! {
    /// Matches a custom emoji
    ///
    /// # Capture Groups:
    /// - `animated`: Whether or not the emoji is animated
    /// - `name`: The emoji's name
    /// - `id`: The emoji's ID
    pub static ref CUSTOM_EMOJI: Regex = Regex::new(&format!(
        r"<(?P<animated>a?):(?P<name>[_a-zA-Z0-9]{{{min_emoji_len},{max_emoji_len}}}):(?P<id>\d{{{min_id_len},{max_id_len}}})>",
        min_id_len = MIN_SNOWFLAKE_LEN,
        max_id_len = MAX_SNOWFLAKE_LEN,
        min_emoji_len = MIN_EMOJI_LEN,
        max_emoji_len = MAX_EMOJI_LEN,
    ))
        .expect("Failed to compile `CUSTOM_EMOJI` regex");

    /// Matches a mentioned user
    ///
    /// # Capture Groups:
    /// - `id`: The mention user's ID
    pub static ref USER_MENTION: Regex = Regex::new(&format!(
            r"<@!?(?P<id>\d{{{min_id_len},{max_id_len}}})>",
            min_id_len = MIN_SNOWFLAKE_LEN,
            max_id_len = MAX_SNOWFLAKE_LEN,
    ))
        .expect("Failed to compile `USER_MENTION` regex");

    /// Matches a mentioned role
    ///
    /// # Capture Groups:
    /// - `id`: The mentioned role's ID
    pub static ref ROLE_MENTION: Regex = Regex::new(&format!(
            r"<@&(?P<id>\d{{{min_id_len},{max_id_len}}})>",
            min_id_len = MIN_SNOWFLAKE_LEN,
            max_id_len = MAX_SNOWFLAKE_LEN,
    ))
        .expect("Failed to compile `ROLE_MENTION` regex");

    /// Matches a slash command
    ///
    /// # Capture Groups:
    /// - `name`: The command name
    /// - `id`: The command ID
    pub static ref SLASH_COMMAND: Regex = Regex::new(&format!(
            r"</(?P<name>[\w-]{{{min_cmd_len},{max_cmd_len}}}):(?P<id>\d{{{min_id_len},{max_id_len}}})>",
            min_id_len = MIN_SNOWFLAKE_LEN,
            max_id_len = MAX_SNOWFLAKE_LEN,
            min_cmd_len = MIN_COMNMAD_LEN,
            max_cmd_len = MAX_COMMAND_LEN,
    ))
        .expect("Failed to compile `SLASH_COMMAND` regex");
}
