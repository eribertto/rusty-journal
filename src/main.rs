// june 18/21 start in new branch regolith
// to complete this exercise

// import cli and tasks modules
mod tasks;
mod cli;
use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() {
    // get the command-line args
	let CommandLineArgs {
		action,
		journal_file,
	} = CommandLineArgs::from_args();

	// unpack the journal
	let journal_file = journal_file.expect("Failed to find journal file");
	// perform the action with match
	match action {
		Add { text } => tasks::add_task(journal_file, Task::new(text)),
		List => tasks::list_tasks(journal_file),
		Done { position } => tasks::complete_task(journal_file, position),
	}
	.expect("Failed to perform action")
}