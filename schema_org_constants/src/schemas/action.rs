/// <https://schema.org/Action>
pub const ACTION_IRI_HTTP: &str = "http://schema.org/Action";
/// <https://schema.org/Action>
pub const ACTION_IRI_HTTPS: &str = "https://schema.org/Action";
/// <https://schema.org/Action>
pub const ACTION_LABEL: &str = "Action";
pub struct ActionIri;
impl PartialEq<&str> for ActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTION_IRI_HTTP || *other == ACTION_IRI_HTTPS
	}
}
impl PartialEq<ActionIri> for &str {
	fn eq(&self, other: &ActionIri) -> bool {
		*self == ACTION_IRI_HTTP || *self == ACTION_IRI_HTTPS
	}
}
pub struct ActionIriOrLabel;
impl PartialEq<&str> for ActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActionIri || *other == ACTION_LABEL
	}
}
impl PartialEq<ActionIriOrLabel> for &str {
	fn eq(&self, other: &ActionIriOrLabel) -> bool {
		*self == ActionIri || *self == ACTION_LABEL
	}
}
