/// <https://schema.org/size>
pub const SIZE_PROPERTY_IRI_HTTP: &str = "http://schema.org/size";
/// <https://schema.org/size>
pub const SIZE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/size";
/// <https://schema.org/size>
pub const SIZE_PROPERTY_LABEL: &str = "size";
pub struct SizePropertyIri;
impl PartialEq<&str> for SizePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIZE_PROPERTY_IRI_HTTP || *other == SIZE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SizePropertyIri> for &str {
	fn eq(&self, other: &SizePropertyIri) -> bool {
		*self == SIZE_PROPERTY_IRI_HTTP || *self == SIZE_PROPERTY_IRI_HTTPS
	}
}
pub struct SizePropertyIriOrLabel;
impl PartialEq<&str> for SizePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SizePropertyIri || *other == SIZE_PROPERTY_LABEL
	}
}
impl PartialEq<SizePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SizePropertyIriOrLabel) -> bool {
		*self == SizePropertyIri || *self == SIZE_PROPERTY_LABEL
	}
}
