/// <https://schema.org/isRelatedTo>
pub const IS_RELATED_TO_PROPERTY_IRI_HTTP: &str = "http://schema.org/isRelatedTo";
/// <https://schema.org/isRelatedTo>
pub const IS_RELATED_TO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isRelatedTo";
/// <https://schema.org/isRelatedTo>
pub const IS_RELATED_TO_PROPERTY_LABEL: &str = "isRelatedTo";
pub struct IsRelatedToPropertyIri;
impl PartialEq<&str> for IsRelatedToPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_RELATED_TO_PROPERTY_IRI_HTTP || *other == IS_RELATED_TO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsRelatedToPropertyIri> for &str {
	fn eq(&self, other: &IsRelatedToPropertyIri) -> bool {
		*self == IS_RELATED_TO_PROPERTY_IRI_HTTP || *self == IS_RELATED_TO_PROPERTY_IRI_HTTPS
	}
}
pub struct IsRelatedToPropertyIriOrLabel;
impl PartialEq<&str> for IsRelatedToPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsRelatedToPropertyIri || *other == IS_RELATED_TO_PROPERTY_LABEL
	}
}
impl PartialEq<IsRelatedToPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsRelatedToPropertyIriOrLabel) -> bool {
		*self == IsRelatedToPropertyIri || *self == IS_RELATED_TO_PROPERTY_LABEL
	}
}
