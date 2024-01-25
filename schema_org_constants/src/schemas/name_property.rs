/// <https://schema.org/name>
pub const NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/name";
/// <https://schema.org/name>
pub const NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/name";
/// <https://schema.org/name>
pub const NAME_PROPERTY_LABEL: &str = "name";
pub struct NamePropertyIri;
impl PartialEq<&str> for NamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NAME_PROPERTY_IRI_HTTP || *other == NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NamePropertyIri> for &str {
	fn eq(&self, other: &NamePropertyIri) -> bool {
		*self == NAME_PROPERTY_IRI_HTTP || *self == NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct NamePropertyIriOrLabel;
impl PartialEq<&str> for NamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NamePropertyIri || *other == NAME_PROPERTY_LABEL
	}
}
impl PartialEq<NamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &NamePropertyIriOrLabel) -> bool {
		*self == NamePropertyIri || *self == NAME_PROPERTY_LABEL
	}
}
