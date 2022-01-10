
fn main() {
	shell_completions::generate();

}

mod shell_completions {
	extern crate structopt;
	use structopt::clap::Shell;
	include!("src/cli.rs");

	pub fn generate() {
		let directory = match std::env::var_os("COMPLETIONS_DIR") {
			None => return,
			Some(out_dir) => out_dir,
		};
		let mut app = KarlArgs::clap();
		app.gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, &directory);
		app.gen_completions(env!("CARGO_PKG_NAME"), Shell::Fish, &directory);
		app.gen_completions(env!("CARGO_PKG_NAME"), Shell::Zsh, &directory);
	}
}
