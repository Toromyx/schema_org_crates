/// <https://schema.org/alternateName>
pub const ALTERNATE_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/alternateName";
/// <https://schema.org/alternateName>
pub const ALTERNATE_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/alternateName";
/// <https://schema.org/alternateName>
pub const ALTERNATE_NAME_PROPERTY_LABEL: &str = "alternateName";
pub struct AlternateNamePropertyIri;
impl PartialEq<&str> for AlternateNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALTERNATE_NAME_PROPERTY_IRI_HTTP || *other == ALTERNATE_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlternateNamePropertyIri> for &str {
	fn eq(&self, other: &AlternateNamePropertyIri) -> bool {
		*self == ALTERNATE_NAME_PROPERTY_IRI_HTTP || *self == ALTERNATE_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct AlternateNamePropertyIriOrLabel;
impl PartialEq<&str> for AlternateNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlternateNamePropertyIri || *other == ALTERNATE_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<AlternateNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlternateNamePropertyIriOrLabel) -> bool {
		*self == AlternateNamePropertyIri || *self == ALTERNATE_NAME_PROPERTY_LABEL
	}
}
