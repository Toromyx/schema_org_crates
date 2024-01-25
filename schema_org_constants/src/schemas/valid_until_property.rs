/// <https://schema.org/validUntil>
pub const VALID_UNTIL_PROPERTY_IRI_HTTP: &str = "http://schema.org/validUntil";
/// <https://schema.org/validUntil>
pub const VALID_UNTIL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/validUntil";
/// <https://schema.org/validUntil>
pub const VALID_UNTIL_PROPERTY_LABEL: &str = "validUntil";
pub struct ValidUntilPropertyIri;
impl PartialEq<&str> for ValidUntilPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALID_UNTIL_PROPERTY_IRI_HTTP || *other == VALID_UNTIL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValidUntilPropertyIri> for &str {
	fn eq(&self, other: &ValidUntilPropertyIri) -> bool {
		*self == VALID_UNTIL_PROPERTY_IRI_HTTP || *self == VALID_UNTIL_PROPERTY_IRI_HTTPS
	}
}
pub struct ValidUntilPropertyIriOrLabel;
impl PartialEq<&str> for ValidUntilPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValidUntilPropertyIri || *other == VALID_UNTIL_PROPERTY_LABEL
	}
}
impl PartialEq<ValidUntilPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValidUntilPropertyIriOrLabel) -> bool {
		*self == ValidUntilPropertyIri || *self == VALID_UNTIL_PROPERTY_LABEL
	}
}
