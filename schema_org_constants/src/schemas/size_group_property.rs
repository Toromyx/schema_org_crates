/// <https://schema.org/sizeGroup>
pub const SIZE_GROUP_PROPERTY_IRI_HTTP: &str = "http://schema.org/sizeGroup";
/// <https://schema.org/sizeGroup>
pub const SIZE_GROUP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sizeGroup";
/// <https://schema.org/sizeGroup>
pub const SIZE_GROUP_PROPERTY_LABEL: &str = "sizeGroup";
pub struct SizeGroupPropertyIri;
impl PartialEq<&str> for SizeGroupPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIZE_GROUP_PROPERTY_IRI_HTTP || *other == SIZE_GROUP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SizeGroupPropertyIri> for &str {
	fn eq(&self, other: &SizeGroupPropertyIri) -> bool {
		*self == SIZE_GROUP_PROPERTY_IRI_HTTP || *self == SIZE_GROUP_PROPERTY_IRI_HTTPS
	}
}
pub struct SizeGroupPropertyIriOrLabel;
impl PartialEq<&str> for SizeGroupPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SizeGroupPropertyIri || *other == SIZE_GROUP_PROPERTY_LABEL
	}
}
impl PartialEq<SizeGroupPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SizeGroupPropertyIriOrLabel) -> bool {
		*self == SizeGroupPropertyIri || *self == SIZE_GROUP_PROPERTY_LABEL
	}
}
