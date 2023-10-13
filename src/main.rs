use clap::Parser;
use github_release_downloader::models::Release;

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

	println!("{res:#?}");

	Ok(())
}
