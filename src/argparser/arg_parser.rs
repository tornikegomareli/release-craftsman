use crate::git_log_format::GitLogFormat;
use chat_gpt_lib_rs::Model;
use clap::{App, Arg};
pub struct ArgParser {}
impl ArgParser {
    pub fn parse_args() -> (
        GitLogFormat,
        Option<String>,
        Option<String>,
        Option<String>,
        Model,
        Option<String>,
    ) {
        let matches = App::new("releasecraftsman")
            .version("0.1.1")
            .arg(
                Arg::with_name("format")
                    .short("f")
                    .long("format")
                    .takes_value(true)
                    .help("Sets the format of git log output (compact|hard)"),
            )
            .arg(
                Arg::with_name("start_tag")
                    .short("s")
                    .long("start")
                    .takes_value(true)
                    .help("Starting tag for the git log"),
            )
            .arg(
                Arg::with_name("end_tag")
                    .short("e")
                    .long("end")
                    .takes_value(true)
                    .help("Ending tag for the git log"),
            )
            .arg(
                Arg::with_name("api_key")
                    .short("k")
                    .long("key")
                    .takes_value(true)
                    .help("API Key for GPT"),
            )
            .arg(
                Arg::with_name("model")
                    .short("m")
                    .long("model")
                    .takes_value(true)
                    .default_value("Gpt_3_5Turbo")
                    .help("GPT model to use (Gpt_3_5Turbo|Gpt_4)"),
            )
            .arg(
                Arg::with_name("version")
                    .short("v")
                    .long("version")
                    .takes_value(true)
                    .help("Version of the release"),
            )
            .get_matches();

        let log_format = match matches.value_of("format").unwrap_or("compact") {
            "compact" => GitLogFormat::Compact,
            "hard" => GitLogFormat::Full,
            _ => GitLogFormat::Compact,
        };

        let start_tag = matches.value_of("start_tag").map(|s| s.to_string());
        let end_tag = matches.value_of("end_tag").map(|e| e.to_string());

        let api_key = matches.value_of("api_key").map(|s| s.to_string());
        let model = match matches.value_of("model").unwrap() {
            "Gpt_4" => Model::Gpt_4,
            "Gpt_3_5Turbo" => Model::Gpt3_5Turbo,
            _ => Model::Gpt3_5Turbo,
        };

        let version = matches.value_of("version").map(|v| v.to_string());

        (log_format, start_tag, end_tag, api_key, model, version)
    }
}
