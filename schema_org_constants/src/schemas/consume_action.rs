/// <https://schema.org/ConsumeAction>
pub const CONSUME_ACTION_IRI_HTTP: &str = "http://schema.org/ConsumeAction";
/// <https://schema.org/ConsumeAction>
pub const CONSUME_ACTION_IRI_HTTPS: &str = "https://schema.org/ConsumeAction";
/// <https://schema.org/ConsumeAction>
pub const CONSUME_ACTION_LABEL: &str = "ConsumeAction";
pub struct ConsumeActionIri;
impl PartialEq<&str> for ConsumeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONSUME_ACTION_IRI_HTTP || *other == CONSUME_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ConsumeActionIri> for &str {
	fn eq(&self, other: &ConsumeActionIri) -> bool {
		*self == CONSUME_ACTION_IRI_HTTP || *self == CONSUME_ACTION_IRI_HTTPS
	}
}
pub struct ConsumeActionIriOrLabel;
impl PartialEq<&str> for ConsumeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ConsumeActionIri || *other == CONSUME_ACTION_LABEL
	}
}
impl PartialEq<ConsumeActionIriOrLabel> for &str {
	fn eq(&self, other: &ConsumeActionIriOrLabel) -> bool {
		*self == ConsumeActionIri || *self == CONSUME_ACTION_LABEL
	}
}
