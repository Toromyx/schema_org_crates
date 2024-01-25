/// <https://schema.org/nonEqual>
pub const NON_EQUAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/nonEqual";
/// <https://schema.org/nonEqual>
pub const NON_EQUAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/nonEqual";
/// <https://schema.org/nonEqual>
pub const NON_EQUAL_PROPERTY_LABEL: &str = "nonEqual";
pub struct NonEqualPropertyIri;
impl PartialEq<&str> for NonEqualPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NON_EQUAL_PROPERTY_IRI_HTTP || *other == NON_EQUAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NonEqualPropertyIri> for &str {
	fn eq(&self, other: &NonEqualPropertyIri) -> bool {
		*self == NON_EQUAL_PROPERTY_IRI_HTTP || *self == NON_EQUAL_PROPERTY_IRI_HTTPS
	}
}
pub struct NonEqualPropertyIriOrLabel;
impl PartialEq<&str> for NonEqualPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NonEqualPropertyIri || *other == NON_EQUAL_PROPERTY_LABEL
	}
}
impl PartialEq<NonEqualPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NonEqualPropertyIriOrLabel) -> bool {
		*self == NonEqualPropertyIri || *self == NON_EQUAL_PROPERTY_LABEL
	}
}
