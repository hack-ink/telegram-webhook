// std
use std::env;
// crates.io
use actix_web::{
	web::{self, Data},
	App, HttpResponse, HttpServer,
};
use anyhow::Result;
use clap::Parser;
use teloxide::{requests::Requester, types::ChatId, Bot as Api};

#[actix_web::main]
async fn main() -> Result<()> {
	color_eyre::install().map_err(|e| anyhow::anyhow!(e))?;
	tracing_subscriber::fmt::init();

	let Cli { port, chat_ids } = Cli::parse();

	Ok(HttpServer::new(move || {
		App::new()
			.app_data(Data::new(Bot {
				api: Api::new(env::var("BOT_TOKEN").expect("BOT_TOKEN env var is not set")),
				chat_ids: chat_ids.iter().cloned().map(ChatId).collect(),
			}))
			.route("/", web::post().to(forward))
	})
	.bind(format!("0.0.0.0:{port}"))?
	.run()
	.await?)
}

#[derive(Debug, Parser)]
#[command(
	version = concat!(
		env!("CARGO_PKG_VERSION"),
		"-",
		env!("VERGEN_GIT_SHA"),
		"-",
		env!("VERGEN_CARGO_TARGET_TRIPLE"),
	),
	about,
	rename_all = "kebab",
)]
struct Cli {
	/// Port to listen on.
	#[arg(long, short, value_name = "PORT", default_value_t = 8080)]
	port: u16,
	/// Telegram chat ID.
	#[arg(
		long,
		short,
		required = true,
		allow_hyphen_values = true,
		value_name = "ID(s)",
		value_delimiter = ','
	)]
	chat_ids: Vec<i64>,
}
#[test]
fn cli_should_work() {
	let cli = Cli::parse_from(["telegram-webhook", "-p", "1234", "-c", "-567,-890"]);

	assert_eq!(cli.port, 1234);
	assert_eq!(cli.chat_ids, vec![-567, -890]);
}

#[derive(Debug)]
struct Bot {
	api: Api,
	chat_ids: Vec<ChatId>,
}

async fn forward(bot: Data<Bot>, message: String) -> HttpResponse {
	tracing::trace!("{message}");

	for i in &bot.chat_ids {
		let r = bot.api.send_message(*i, &message).await;

		tracing::trace!("{r:?}");
	}

	HttpResponse::Ok().finish()
}
