/// <https://schema.org/description>
pub const DESCRIPTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/description";
/// <https://schema.org/description>
pub const DESCRIPTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/description";
/// <https://schema.org/description>
pub const DESCRIPTION_PROPERTY_LABEL: &str = "description";
pub struct DescriptionPropertyIri;
impl PartialEq<&str> for DescriptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DESCRIPTION_PROPERTY_IRI_HTTP || *other == DESCRIPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DescriptionPropertyIri> for &str {
	fn eq(&self, other: &DescriptionPropertyIri) -> bool {
		*self == DESCRIPTION_PROPERTY_IRI_HTTP || *self == DESCRIPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct DescriptionPropertyIriOrLabel;
impl PartialEq<&str> for DescriptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DescriptionPropertyIri || *other == DESCRIPTION_PROPERTY_LABEL
	}
}
impl PartialEq<DescriptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DescriptionPropertyIriOrLabel) -> bool {
		*self == DescriptionPropertyIri || *self == DESCRIPTION_PROPERTY_LABEL
	}
}
