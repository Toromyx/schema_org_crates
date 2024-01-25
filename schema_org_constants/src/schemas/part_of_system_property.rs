/// <https://schema.org/partOfSystem>
pub const PART_OF_SYSTEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/partOfSystem";
/// <https://schema.org/partOfSystem>
pub const PART_OF_SYSTEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/partOfSystem";
/// <https://schema.org/partOfSystem>
pub const PART_OF_SYSTEM_PROPERTY_LABEL: &str = "partOfSystem";
pub struct PartOfSystemPropertyIri;
impl PartialEq<&str> for PartOfSystemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PART_OF_SYSTEM_PROPERTY_IRI_HTTP || *other == PART_OF_SYSTEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PartOfSystemPropertyIri> for &str {
	fn eq(&self, other: &PartOfSystemPropertyIri) -> bool {
		*self == PART_OF_SYSTEM_PROPERTY_IRI_HTTP || *self == PART_OF_SYSTEM_PROPERTY_IRI_HTTPS
	}
}
pub struct PartOfSystemPropertyIriOrLabel;
impl PartialEq<&str> for PartOfSystemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PartOfSystemPropertyIri || *other == PART_OF_SYSTEM_PROPERTY_LABEL
	}
}
impl PartialEq<PartOfSystemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PartOfSystemPropertyIriOrLabel) -> bool {
		*self == PartOfSystemPropertyIri || *self == PART_OF_SYSTEM_PROPERTY_LABEL
	}
}
