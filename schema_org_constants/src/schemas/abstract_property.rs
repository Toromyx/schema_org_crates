/// <https://schema.org/abstract>
pub const ABSTRACT_PROPERTY_IRI_HTTP: &str = "http://schema.org/abstract";
/// <https://schema.org/abstract>
pub const ABSTRACT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/abstract";
/// <https://schema.org/abstract>
pub const ABSTRACT_PROPERTY_LABEL: &str = "abstract";
pub struct AbstractPropertyIri;
impl PartialEq<&str> for AbstractPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ABSTRACT_PROPERTY_IRI_HTTP || *other == ABSTRACT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AbstractPropertyIri> for &str {
	fn eq(&self, other: &AbstractPropertyIri) -> bool {
		*self == ABSTRACT_PROPERTY_IRI_HTTP || *self == ABSTRACT_PROPERTY_IRI_HTTPS
	}
}
pub struct AbstractPropertyIriOrLabel;
impl PartialEq<&str> for AbstractPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AbstractPropertyIri || *other == ABSTRACT_PROPERTY_LABEL
	}
}
impl PartialEq<AbstractPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AbstractPropertyIriOrLabel) -> bool {
		*self == AbstractPropertyIri || *self == ABSTRACT_PROPERTY_LABEL
	}
}
