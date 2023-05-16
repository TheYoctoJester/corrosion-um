pub mod invocation {
	
use std::path::PathBuf;

const ERR_NOT_ENOUGH_ARGUMENTS: &str = "not enough arguments";
const ERR_ACTION_NOT_RECOGNIZED: &str = "action string not recognized";

#[derive(Debug, PartialEq)]
pub enum Action {
	Download,
	ArtifactInstall,
	ArtifactReboot,
	ArtifactVerifyReboot,
	ArtifactCommit,
	Cleanup,
	ArtifactRollback,
	ArtifactRollbackReboot,
	ArtifactFailure,
	SupportsRollback,
	NeedsArtifactReboot,
	SupportsAugmentedArtifacts,
	ListSupportedOriginalTypes,
	PermittedAugmentedHeaders
}

#[derive(Debug)]
pub struct Invocation {
	pub action: Action,
	pub path: PathBuf // this owns the data according to https://stackoverflow.com/questions/32730714/what-is-the-right-way-to-store-an-immutable-path-in-a-struct
}

impl Invocation {
	pub fn build(args: &[String]) -> Result<Invocation, &'static str> {
		// we always get two arguments!
		if args.len() < 3 {
			return Err(ERR_NOT_ENOUGH_ARGUMENTS)
		}

		let action: Action = match args[1].as_str() {
			"Download" => Action::Download,
			"ArtifactInstall" => Action::Download,
			"ArtifactReboot" => Action::Download,
			"ArtifactVerifyReboot" => Action::Download,
			"ArtifactCommit" => Action::Download,
			"Cleanup" => Action::Download,
			"ArtifactRollback" => Action::Download,
			"ArtifactRollbackReboot" => Action::Download,
			"ArtifactFailure" => Action::Download,
			"SupportsRollback" => Action::Download,
			"NeedsArtifactReboot" => Action::Download,
			"SupportsAugmentedArtifacts" => Action::Download,
			"ListSupportedOriginalTypes" => Action::Download,
			"PermittedAugmentedHeaders" => Action::Download,
			_ => return Err(ERR_ACTION_NOT_RECOGNIZED)
		};

		Ok(Invocation{action, path: PathBuf::from(args[2].clone())})
	}
}

}