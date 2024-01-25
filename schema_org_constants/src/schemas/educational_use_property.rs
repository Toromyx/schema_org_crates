/// <https://schema.org/educationalUse>
pub const EDUCATIONAL_USE_PROPERTY_IRI_HTTP: &str = "http://schema.org/educationalUse";
/// <https://schema.org/educationalUse>
pub const EDUCATIONAL_USE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/educationalUse";
/// <https://schema.org/educationalUse>
pub const EDUCATIONAL_USE_PROPERTY_LABEL: &str = "educationalUse";
pub struct EducationalUsePropertyIri;
impl PartialEq<&str> for EducationalUsePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_USE_PROPERTY_IRI_HTTP || *other == EDUCATIONAL_USE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EducationalUsePropertyIri> for &str {
	fn eq(&self, other: &EducationalUsePropertyIri) -> bool {
		*self == EDUCATIONAL_USE_PROPERTY_IRI_HTTP || *self == EDUCATIONAL_USE_PROPERTY_IRI_HTTPS
	}
}
pub struct EducationalUsePropertyIriOrLabel;
impl PartialEq<&str> for EducationalUsePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalUsePropertyIri || *other == EDUCATIONAL_USE_PROPERTY_LABEL
	}
}
impl PartialEq<EducationalUsePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EducationalUsePropertyIriOrLabel) -> bool {
		*self == EducationalUsePropertyIri || *self == EDUCATIONAL_USE_PROPERTY_LABEL
	}
}
