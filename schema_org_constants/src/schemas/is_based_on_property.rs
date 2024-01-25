/// <https://schema.org/isBasedOn>
pub const IS_BASED_ON_PROPERTY_IRI_HTTP: &str = "http://schema.org/isBasedOn";
/// <https://schema.org/isBasedOn>
pub const IS_BASED_ON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isBasedOn";
/// <https://schema.org/isBasedOn>
pub const IS_BASED_ON_PROPERTY_LABEL: &str = "isBasedOn";
pub struct IsBasedOnPropertyIri;
impl PartialEq<&str> for IsBasedOnPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_BASED_ON_PROPERTY_IRI_HTTP || *other == IS_BASED_ON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsBasedOnPropertyIri> for &str {
	fn eq(&self, other: &IsBasedOnPropertyIri) -> bool {
		*self == IS_BASED_ON_PROPERTY_IRI_HTTP || *self == IS_BASED_ON_PROPERTY_IRI_HTTPS
	}
}
pub struct IsBasedOnPropertyIriOrLabel;
impl PartialEq<&str> for IsBasedOnPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsBasedOnPropertyIri || *other == IS_BASED_ON_PROPERTY_LABEL
	}
}
impl PartialEq<IsBasedOnPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsBasedOnPropertyIriOrLabel) -> bool {
		*self == IsBasedOnPropertyIri || *self == IS_BASED_ON_PROPERTY_LABEL
	}
}
