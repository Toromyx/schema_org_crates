/// <https://schema.org/MoveAction>
pub const MOVE_ACTION_IRI_HTTP: &str = "http://schema.org/MoveAction";
/// <https://schema.org/MoveAction>
pub const MOVE_ACTION_IRI_HTTPS: &str = "https://schema.org/MoveAction";
/// <https://schema.org/MoveAction>
pub const MOVE_ACTION_LABEL: &str = "MoveAction";
pub struct MoveActionIri;
impl PartialEq<&str> for MoveActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOVE_ACTION_IRI_HTTP || *other == MOVE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<MoveActionIri> for &str {
	fn eq(&self, other: &MoveActionIri) -> bool {
		*self == MOVE_ACTION_IRI_HTTP || *self == MOVE_ACTION_IRI_HTTPS
	}
}
pub struct MoveActionIriOrLabel;
impl PartialEq<&str> for MoveActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MoveActionIri || *other == MOVE_ACTION_LABEL
	}
}
impl PartialEq<MoveActionIriOrLabel> for &str {
	fn eq(&self, other: &MoveActionIriOrLabel) -> bool {
		*self == MoveActionIri || *self == MOVE_ACTION_LABEL
	}
}
