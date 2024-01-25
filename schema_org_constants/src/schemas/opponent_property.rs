/// <https://schema.org/opponent>
pub const OPPONENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/opponent";
/// <https://schema.org/opponent>
pub const OPPONENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/opponent";
/// <https://schema.org/opponent>
pub const OPPONENT_PROPERTY_LABEL: &str = "opponent";
pub struct OpponentPropertyIri;
impl PartialEq<&str> for OpponentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPPONENT_PROPERTY_IRI_HTTP || *other == OPPONENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OpponentPropertyIri> for &str {
	fn eq(&self, other: &OpponentPropertyIri) -> bool {
		*self == OPPONENT_PROPERTY_IRI_HTTP || *self == OPPONENT_PROPERTY_IRI_HTTPS
	}
}
pub struct OpponentPropertyIriOrLabel;
impl PartialEq<&str> for OpponentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OpponentPropertyIri || *other == OPPONENT_PROPERTY_LABEL
	}
}
impl PartialEq<OpponentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OpponentPropertyIriOrLabel) -> bool {
		*self == OpponentPropertyIri || *self == OPPONENT_PROPERTY_LABEL
	}
}
