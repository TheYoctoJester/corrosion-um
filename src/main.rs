use std::env;
use std::process;

pub mod mender_update_module;

use crate::mender_update_module::invocation::Action;
use crate::mender_update_module::invocation::Invocation;


fn main() {
	let args: Vec<String> = env::args().collect();
    let i = Invocation::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

	match i.action {
		Action::Download => (),
		Action::ArtifactInstall => (),
		Action::ArtifactReboot => (),
		Action::ArtifactVerifyReboot => (),
		Action::ArtifactCommit => (),
		Action::Cleanup => (),
		Action::ArtifactRollback => (),
		Action::ArtifactRollbackReboot => (),
		Action::ArtifactFailure => (),
		Action::SupportsRollback => (),
		Action::NeedsArtifactReboot => (),
		Action::SupportsAugmentedArtifacts => (),
		Action::ListSupportedOriginalTypes => (),
		Action::PermittedAugmentedHeaders => ()
	}
}