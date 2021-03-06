use serde::{Deserialize, Serialize};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::regex;

#[derive(Serialize)]
struct PlaygroundBody {
    backtrace: bool,
    channel: String,
    code: String,
    #[serde(rename = "crateType")]
    crate_type: String,
    edition: String,
    mode: String,
    tests: bool,
}

#[derive(Debug, Deserialize)]
struct PlaygroundResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

#[command("eval")]
#[aliases("playground", "rust-playground")]
#[min_args(1)]
#[description = "Execute Rust (nightly) code. Note: this code is NOT executed \
        in context, it is executed by the \
        [Rust Playground](https://play.rust-lang.org), thereby being available \
        to all users"]
pub async fn eval_cmd(
    ctx: &Context,
    msg: &Message,
    mut args: Args,
) -> CommandResult {
    let first_arg = args.single::<String>()?;

    let implicit_main = first_arg.to_lowercase().trim() != "--no-implicit-main";

    let parsed_code = if implicit_main {
        let code = args.raw().collect::<Vec<&str>>().join(" ");
        let parsed = parse_code(&code);

        format!("fn main() {{\n{}\n}}", parsed)
    } else {
        let code = args.remains().unwrap().to_string();
        parse_code(&code)
    };

    let m = match execute(&parsed_code).await {
        Ok(output) => {
            if output.success {
                format!(
                    "`stdout:` ```rs\n{}\n```",
                    output
                        .stdout
                        .get(0..2000)
                        .unwrap_or(&output.stdout)
                        .to_string(),
                )
            } else {
                format!(
                    "`stderr:` ```rs\n{}\n```",
                    output
                        .stderr
                        .get(0..2000)
                        .unwrap_or(&output.stderr)
                        .to_string(),
                )
            }
        },
        Err(err) => {
            format!("An error occurred :c ```rs\n{:#?}```", err)
        },
    };

    msg.channel_id.say(&ctx.http, m).await?;

    Ok(())
}

fn parse_code(code: &str) -> String {
    if let Ok(Some(caps)) = regex::CODE_BLOCK.captures(&code) {
        if let Some(c) = caps.name("code") {
            c.as_str().trim().to_string()
        } else {
            code.to_string()
        }
    } else {
        code.to_string()
    }
}

async fn execute(code: &str) -> Result<PlaygroundResponse, reqwest::Error> {
    let client = reqwest::Client::new();

    let body = PlaygroundBody {
        backtrace: false,
        channel: "nightly".to_string(),
        code: code.to_string(),
        crate_type: "bin".to_string(),
        edition: "2018".to_string(),
        mode: "debug".to_string(),
        tests: false,
    };

    let result: PlaygroundResponse = client
        .post("https://play.rust-lang.org/execute")
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_code_with_code_block() {
        let code_block = r#"```rs
let num = 1 + 1;
println!("{}", num);
            ```"#
            .to_string();

        let expected = r#"let num = 1 + 1;
println!("{}", num);"#
            .to_string();

        let parsed = parse_code(&code_block);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn parse_code_without_code_block() {
        let code = r#"println!("{}", a);"#.to_string();
        let expected = r#"println!("{}", a);"#.to_string();

        let parsed = parse_code(&code);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn parse_code_with_code_block_inside() {
        let code = r#"println!("```rs\n{}```", 1 + 1);"#.to_string();
        let expected = r#"println!("```rs\n{}```", 1 + 1);"#.to_string();

        let parsed = parse_code(&code);

        assert_eq!(parsed, expected);
    }
}
