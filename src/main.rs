use clap::Parser;
use github_release_downloader::models::Release;
use inquire::{MultiSelect, Select};

#[derive(Parser)]
struct Cli {
	repo: String,

	#[arg(short, long)]
	token: String,
}

fn fetch_releases(repo: &str, token: &str) -> anyhow::Result<Vec<Release>> {
	Ok(ureq::get(&format!(
		"https://api.github.com/repos/{repo}/releases"
	))
	.set("Authorization", token)
	.call()?
	.into_json::<Vec<Release>>()?)
}

fn main() -> anyhow::Result<()> {
	let cli = Cli::parse();

	let res = fetch_releases(&cli.repo, &cli.token)?;

	let rel = Select::new("choose release", res)
		.with_page_size(8)
		.prompt()?;

	let ans = MultiSelect::new(
		"choose asset(s) to download",
		rel.assets,
	)
	.with_page_size(10)
	.prompt();

	println!("{ans:#?}");

	Ok(())
}
