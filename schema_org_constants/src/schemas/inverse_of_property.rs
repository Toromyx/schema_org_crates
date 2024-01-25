/// <https://schema.org/inverseOf>
pub const INVERSE_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/inverseOf";
/// <https://schema.org/inverseOf>
pub const INVERSE_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inverseOf";
/// <https://schema.org/inverseOf>
pub const INVERSE_OF_PROPERTY_LABEL: &str = "inverseOf";
pub struct InverseOfPropertyIri;
impl PartialEq<&str> for InverseOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INVERSE_OF_PROPERTY_IRI_HTTP || *other == INVERSE_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InverseOfPropertyIri> for &str {
	fn eq(&self, other: &InverseOfPropertyIri) -> bool {
		*self == INVERSE_OF_PROPERTY_IRI_HTTP || *self == INVERSE_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct InverseOfPropertyIriOrLabel;
impl PartialEq<&str> for InverseOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InverseOfPropertyIri || *other == INVERSE_OF_PROPERTY_LABEL
	}
}
impl PartialEq<InverseOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InverseOfPropertyIriOrLabel) -> bool {
		*self == InverseOfPropertyIri || *self == INVERSE_OF_PROPERTY_LABEL
	}
}
