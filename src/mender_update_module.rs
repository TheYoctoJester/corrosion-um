pub mod invocation {
	
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::EnumString;

const ERR_NOT_ENOUGH_ARGUMENTS: &str = "not enough arguments";
const ERR_ACTION_NOT_RECOGNIZED: &str = "action string not recognized";

#[derive(Debug, PartialEq, EnumString)]
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
		// first try: an information request, because that only requires one argument
		if args.len() < 3 {
			return Err(ERR_NOT_ENOUGH_ARGUMENTS)
		}

		let action: Action = match Action::from_str(&args[1]) {
			Err(_) => return Err(ERR_ACTION_NOT_RECOGNIZED),
			Ok(a) => a
		};

		Ok(Invocation{action, path: PathBuf::from(args[2].clone())})
	}
}

}