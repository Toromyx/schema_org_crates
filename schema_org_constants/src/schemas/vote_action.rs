/// <https://schema.org/VoteAction>
pub const VOTE_ACTION_IRI_HTTP: &str = "http://schema.org/VoteAction";
/// <https://schema.org/VoteAction>
pub const VOTE_ACTION_IRI_HTTPS: &str = "https://schema.org/VoteAction";
/// <https://schema.org/VoteAction>
pub const VOTE_ACTION_LABEL: &str = "VoteAction";
pub struct VoteActionIri;
impl PartialEq<&str> for VoteActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VOTE_ACTION_IRI_HTTP || *other == VOTE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<VoteActionIri> for &str {
	fn eq(&self, other: &VoteActionIri) -> bool {
		*self == VOTE_ACTION_IRI_HTTP || *self == VOTE_ACTION_IRI_HTTPS
	}
}
pub struct VoteActionIriOrLabel;
impl PartialEq<&str> for VoteActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VoteActionIri || *other == VOTE_ACTION_LABEL
	}
}
impl PartialEq<VoteActionIriOrLabel> for &str {
	fn eq(&self, other: &VoteActionIriOrLabel) -> bool {
		*self == VoteActionIri || *self == VOTE_ACTION_LABEL
	}
}
