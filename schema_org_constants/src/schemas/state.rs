/// <https://schema.org/State>
pub const STATE_IRI_HTTP: &str = "http://schema.org/State";
/// <https://schema.org/State>
pub const STATE_IRI_HTTPS: &str = "https://schema.org/State";
/// <https://schema.org/State>
pub const STATE_LABEL: &str = "State";
pub struct StateIri;
impl PartialEq<&str> for StateIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STATE_IRI_HTTP || *other == STATE_IRI_HTTPS
	}
}
impl PartialEq<StateIri> for &str {
	fn eq(&self, other: &StateIri) -> bool {
		*self == STATE_IRI_HTTP || *self == STATE_IRI_HTTPS
	}
}
pub struct StateIriOrLabel;
impl PartialEq<&str> for StateIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StateIri || *other == STATE_LABEL
	}
}
impl PartialEq<StateIriOrLabel> for &str {
	fn eq(&self, other: &StateIriOrLabel) -> bool {
		*self == StateIri || *self == STATE_LABEL
	}
}
