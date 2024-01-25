/// <https://schema.org/alternativeOf>
pub const ALTERNATIVE_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/alternativeOf";
/// <https://schema.org/alternativeOf>
pub const ALTERNATIVE_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/alternativeOf";
/// <https://schema.org/alternativeOf>
pub const ALTERNATIVE_OF_PROPERTY_LABEL: &str = "alternativeOf";
pub struct AlternativeOfPropertyIri;
impl PartialEq<&str> for AlternativeOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALTERNATIVE_OF_PROPERTY_IRI_HTTP || *other == ALTERNATIVE_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlternativeOfPropertyIri> for &str {
	fn eq(&self, other: &AlternativeOfPropertyIri) -> bool {
		*self == ALTERNATIVE_OF_PROPERTY_IRI_HTTP || *self == ALTERNATIVE_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct AlternativeOfPropertyIriOrLabel;
impl PartialEq<&str> for AlternativeOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlternativeOfPropertyIri || *other == ALTERNATIVE_OF_PROPERTY_LABEL
	}
}
impl PartialEq<AlternativeOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlternativeOfPropertyIriOrLabel) -> bool {
		*self == AlternativeOfPropertyIri || *self == ALTERNATIVE_OF_PROPERTY_LABEL
	}
}
