/// <https://schema.org/equal>
pub const EQUAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/equal";
/// <https://schema.org/equal>
pub const EQUAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/equal";
/// <https://schema.org/equal>
pub const EQUAL_PROPERTY_LABEL: &str = "equal";
pub struct EqualPropertyIri;
impl PartialEq<&str> for EqualPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EQUAL_PROPERTY_IRI_HTTP || *other == EQUAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EqualPropertyIri> for &str {
	fn eq(&self, other: &EqualPropertyIri) -> bool {
		*self == EQUAL_PROPERTY_IRI_HTTP || *self == EQUAL_PROPERTY_IRI_HTTPS
	}
}
pub struct EqualPropertyIriOrLabel;
impl PartialEq<&str> for EqualPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EqualPropertyIri || *other == EQUAL_PROPERTY_LABEL
	}
}
impl PartialEq<EqualPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EqualPropertyIriOrLabel) -> bool {
		*self == EqualPropertyIri || *self == EQUAL_PROPERTY_LABEL
	}
}
