/// <https://schema.org/isPartOf>
pub const IS_PART_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/isPartOf";
/// <https://schema.org/isPartOf>
pub const IS_PART_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isPartOf";
/// <https://schema.org/isPartOf>
pub const IS_PART_OF_PROPERTY_LABEL: &str = "isPartOf";
pub struct IsPartOfPropertyIri;
impl PartialEq<&str> for IsPartOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_PART_OF_PROPERTY_IRI_HTTP || *other == IS_PART_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsPartOfPropertyIri> for &str {
	fn eq(&self, other: &IsPartOfPropertyIri) -> bool {
		*self == IS_PART_OF_PROPERTY_IRI_HTTP || *self == IS_PART_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct IsPartOfPropertyIriOrLabel;
impl PartialEq<&str> for IsPartOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsPartOfPropertyIri || *other == IS_PART_OF_PROPERTY_LABEL
	}
}
impl PartialEq<IsPartOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsPartOfPropertyIriOrLabel) -> bool {
		*self == IsPartOfPropertyIri || *self == IS_PART_OF_PROPERTY_LABEL
	}
}
