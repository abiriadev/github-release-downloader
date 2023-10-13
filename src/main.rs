use std::{
	fs::File,
	io::{copy, Read},
};

use clap::Parser;
use github_release_downloader::models::Release;
use indicatif::{MultiProgress, ProgressBar};
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

struct PgReader<T>(T, ProgressBar)
where T: Read;

impl<T: Read> Read for PgReader<T> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let res = self.0.read(buf);

		if let Ok(len) = res {
			self.1.inc(len as u64);
		}

		res
	}
}

fn download(url: &str, filename: &str, pg: ProgressBar) -> anyhow::Result<()> {
	let res = ureq::get(url)
		.set("Accept", "application/octet-stream")
		.call()?;

	let len = res
		.header("Content-Length")
		.expect("content length does not exist");

	println!("{filename} len: {len}");

	let reader = res.into_reader();
	let mut reader = PgReader(reader, pg.clone());
	let mut f = File::create(filename)?;

	copy(&mut reader, &mut f)?;

	pg.finish_with_message(format!("{filename} done"));

	Ok(())
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
	.prompt()?;

	let mpg = MultiProgress::new();

	ans.into_iter().for_each(|ass| {
		let pg = ProgressBar::new(ass.size as u64);
		mpg.add(pg.clone());

		download(&ass.url, &ass.name, pg).unwrap();
	});

	Ok(())
}
