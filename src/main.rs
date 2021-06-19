// june 18/21 start in new branch regolith
// to complete this exercise

// import cli and tasks modules
use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;
mod tasks;
mod cli;

// new function to use the home crate in Cargo.toml
fn find_default_journal_file() -> Option<PathBuf> {
	home::home_dir().map(|mut path| {
		path.push(".rusty-journal.json");
		path
	})
}

// fn main to return the type anyhow::Result<()>
fn main() -> anyhow::Result<()> {
    // get the command-line args
	let CommandLineArgs {
		action,
		journal_file,
	} = CommandLineArgs::from_args();

	// new version
	let journal_file = journal_file
	.or_else(find_default_journal_file)
	.ok_or(anyhow!("Failed to find journal file."))?;


	// new match block, note the symbol ? after the closing curly
	// to remove the expect calls from our code
	match action {
		Add { text } => tasks::add_task(journal_file, Task::new(text)),
		List => tasks::list_tasks(journal_file),
		Done { position } => tasks::complete_task(journal_file, position),
	}?;
	Ok(())
}