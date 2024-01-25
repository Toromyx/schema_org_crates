/// <https://schema.org/AgreeAction>
pub const AGREE_ACTION_IRI_HTTP: &str = "http://schema.org/AgreeAction";
/// <https://schema.org/AgreeAction>
pub const AGREE_ACTION_IRI_HTTPS: &str = "https://schema.org/AgreeAction";
/// <https://schema.org/AgreeAction>
pub const AGREE_ACTION_LABEL: &str = "AgreeAction";
pub struct AgreeActionIri;
impl PartialEq<&str> for AgreeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AGREE_ACTION_IRI_HTTP || *other == AGREE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<AgreeActionIri> for &str {
	fn eq(&self, other: &AgreeActionIri) -> bool {
		*self == AGREE_ACTION_IRI_HTTP || *self == AGREE_ACTION_IRI_HTTPS
	}
}
pub struct AgreeActionIriOrLabel;
impl PartialEq<&str> for AgreeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AgreeActionIri || *other == AGREE_ACTION_LABEL
	}
}
impl PartialEq<AgreeActionIriOrLabel> for &str {
	fn eq(&self, other: &AgreeActionIriOrLabel) -> bool {
		*self == AgreeActionIri || *self == AGREE_ACTION_LABEL
	}
}
