/// <https://schema.org/DisagreeAction>
pub const DISAGREE_ACTION_IRI_HTTP: &str = "http://schema.org/DisagreeAction";
/// <https://schema.org/DisagreeAction>
pub const DISAGREE_ACTION_IRI_HTTPS: &str = "https://schema.org/DisagreeAction";
/// <https://schema.org/DisagreeAction>
pub const DISAGREE_ACTION_LABEL: &str = "DisagreeAction";
pub struct DisagreeActionIri;
impl PartialEq<&str> for DisagreeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISAGREE_ACTION_IRI_HTTP || *other == DISAGREE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<DisagreeActionIri> for &str {
	fn eq(&self, other: &DisagreeActionIri) -> bool {
		*self == DISAGREE_ACTION_IRI_HTTP || *self == DISAGREE_ACTION_IRI_HTTPS
	}
}
pub struct DisagreeActionIriOrLabel;
impl PartialEq<&str> for DisagreeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DisagreeActionIri || *other == DISAGREE_ACTION_LABEL
	}
}
impl PartialEq<DisagreeActionIriOrLabel> for &str {
	fn eq(&self, other: &DisagreeActionIriOrLabel) -> bool {
		*self == DisagreeActionIri || *self == DISAGREE_ACTION_LABEL
	}
}
