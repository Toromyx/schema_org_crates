/// <https://schema.org/LoseAction>
pub const LOSE_ACTION_IRI_HTTP: &str = "http://schema.org/LoseAction";
/// <https://schema.org/LoseAction>
pub const LOSE_ACTION_IRI_HTTPS: &str = "https://schema.org/LoseAction";
/// <https://schema.org/LoseAction>
pub const LOSE_ACTION_LABEL: &str = "LoseAction";
pub struct LoseActionIri;
impl PartialEq<&str> for LoseActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOSE_ACTION_IRI_HTTP || *other == LOSE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<LoseActionIri> for &str {
	fn eq(&self, other: &LoseActionIri) -> bool {
		*self == LOSE_ACTION_IRI_HTTP || *self == LOSE_ACTION_IRI_HTTPS
	}
}
pub struct LoseActionIriOrLabel;
impl PartialEq<&str> for LoseActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LoseActionIri || *other == LOSE_ACTION_LABEL
	}
}
impl PartialEq<LoseActionIriOrLabel> for &str {
	fn eq(&self, other: &LoseActionIriOrLabel) -> bool {
		*self == LoseActionIri || *self == LOSE_ACTION_LABEL
	}
}
