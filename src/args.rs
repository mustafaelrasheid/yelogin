use clap::Parser;

#[derive(Parser)]
#[command(name = "yelogin")]
#[command(version)]
#[command(author = "mustafaelrasheid")]
#[command(
	about = "A simple login and authentication program.",
	long_about = None
)]
pub struct Cli {
	pub path: Option<String>,
	#[arg(long)]
	pub set_xdg_runtime_dir: bool,
}
