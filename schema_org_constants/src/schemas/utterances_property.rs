/// <https://schema.org/utterances>
pub const UTTERANCES_PROPERTY_IRI_HTTP: &str = "http://schema.org/utterances";
/// <https://schema.org/utterances>
pub const UTTERANCES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/utterances";
/// <https://schema.org/utterances>
pub const UTTERANCES_PROPERTY_LABEL: &str = "utterances";
pub struct UtterancesPropertyIri;
impl PartialEq<&str> for UtterancesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UTTERANCES_PROPERTY_IRI_HTTP || *other == UTTERANCES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UtterancesPropertyIri> for &str {
	fn eq(&self, other: &UtterancesPropertyIri) -> bool {
		*self == UTTERANCES_PROPERTY_IRI_HTTP || *self == UTTERANCES_PROPERTY_IRI_HTTPS
	}
}
pub struct UtterancesPropertyIriOrLabel;
impl PartialEq<&str> for UtterancesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UtterancesPropertyIri || *other == UTTERANCES_PROPERTY_LABEL
	}
}
impl PartialEq<UtterancesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &UtterancesPropertyIriOrLabel) -> bool {
		*self == UtterancesPropertyIri || *self == UTTERANCES_PROPERTY_LABEL
	}
}
