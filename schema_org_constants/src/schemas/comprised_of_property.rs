/// <https://schema.org/comprisedOf>
pub const COMPRISED_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/comprisedOf";
/// <https://schema.org/comprisedOf>
pub const COMPRISED_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/comprisedOf";
/// <https://schema.org/comprisedOf>
pub const COMPRISED_OF_PROPERTY_LABEL: &str = "comprisedOf";
pub struct ComprisedOfPropertyIri;
impl PartialEq<&str> for ComprisedOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPRISED_OF_PROPERTY_IRI_HTTP || *other == COMPRISED_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ComprisedOfPropertyIri> for &str {
	fn eq(&self, other: &ComprisedOfPropertyIri) -> bool {
		*self == COMPRISED_OF_PROPERTY_IRI_HTTP || *self == COMPRISED_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct ComprisedOfPropertyIriOrLabel;
impl PartialEq<&str> for ComprisedOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ComprisedOfPropertyIri || *other == COMPRISED_OF_PROPERTY_LABEL
	}
}
impl PartialEq<ComprisedOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ComprisedOfPropertyIriOrLabel) -> bool {
		*self == ComprisedOfPropertyIri || *self == COMPRISED_OF_PROPERTY_LABEL
	}
}
