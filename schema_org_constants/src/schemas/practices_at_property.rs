/// <https://schema.org/practicesAt>
pub const PRACTICES_AT_PROPERTY_IRI_HTTP: &str = "http://schema.org/practicesAt";
/// <https://schema.org/practicesAt>
pub const PRACTICES_AT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/practicesAt";
/// <https://schema.org/practicesAt>
pub const PRACTICES_AT_PROPERTY_LABEL: &str = "practicesAt";
pub struct PracticesAtPropertyIri;
impl PartialEq<&str> for PracticesAtPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRACTICES_AT_PROPERTY_IRI_HTTP || *other == PRACTICES_AT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PracticesAtPropertyIri> for &str {
	fn eq(&self, other: &PracticesAtPropertyIri) -> bool {
		*self == PRACTICES_AT_PROPERTY_IRI_HTTP || *self == PRACTICES_AT_PROPERTY_IRI_HTTPS
	}
}
pub struct PracticesAtPropertyIriOrLabel;
impl PartialEq<&str> for PracticesAtPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PracticesAtPropertyIri || *other == PRACTICES_AT_PROPERTY_LABEL
	}
}
impl PartialEq<PracticesAtPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PracticesAtPropertyIriOrLabel) -> bool {
		*self == PracticesAtPropertyIri || *self == PRACTICES_AT_PROPERTY_LABEL
	}
}
