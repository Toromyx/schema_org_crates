/// <https://schema.org/validIn>
pub const VALID_IN_PROPERTY_IRI_HTTP: &str = "http://schema.org/validIn";
/// <https://schema.org/validIn>
pub const VALID_IN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/validIn";
/// <https://schema.org/validIn>
pub const VALID_IN_PROPERTY_LABEL: &str = "validIn";
pub struct ValidInPropertyIri;
impl PartialEq<&str> for ValidInPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALID_IN_PROPERTY_IRI_HTTP || *other == VALID_IN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValidInPropertyIri> for &str {
	fn eq(&self, other: &ValidInPropertyIri) -> bool {
		*self == VALID_IN_PROPERTY_IRI_HTTP || *self == VALID_IN_PROPERTY_IRI_HTTPS
	}
}
pub struct ValidInPropertyIriOrLabel;
impl PartialEq<&str> for ValidInPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValidInPropertyIri || *other == VALID_IN_PROPERTY_LABEL
	}
}
impl PartialEq<ValidInPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValidInPropertyIriOrLabel) -> bool {
		*self == ValidInPropertyIri || *self == VALID_IN_PROPERTY_LABEL
	}
}
