/// <https://schema.org/knows>
pub const KNOWS_PROPERTY_IRI_HTTP: &str = "http://schema.org/knows";
/// <https://schema.org/knows>
pub const KNOWS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/knows";
/// <https://schema.org/knows>
pub const KNOWS_PROPERTY_LABEL: &str = "knows";
pub struct KnowsPropertyIri;
impl PartialEq<&str> for KnowsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == KNOWS_PROPERTY_IRI_HTTP || *other == KNOWS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<KnowsPropertyIri> for &str {
	fn eq(&self, other: &KnowsPropertyIri) -> bool {
		*self == KNOWS_PROPERTY_IRI_HTTP || *self == KNOWS_PROPERTY_IRI_HTTPS
	}
}
pub struct KnowsPropertyIriOrLabel;
impl PartialEq<&str> for KnowsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == KnowsPropertyIri || *other == KNOWS_PROPERTY_LABEL
	}
}
impl PartialEq<KnowsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &KnowsPropertyIriOrLabel) -> bool {
		*self == KnowsPropertyIri || *self == KNOWS_PROPERTY_LABEL
	}
}
