/// <https://schema.org/validFrom>
pub const VALID_FROM_PROPERTY_IRI_HTTP: &str = "http://schema.org/validFrom";
/// <https://schema.org/validFrom>
pub const VALID_FROM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/validFrom";
/// <https://schema.org/validFrom>
pub const VALID_FROM_PROPERTY_LABEL: &str = "validFrom";
pub struct ValidFromPropertyIri;
impl PartialEq<&str> for ValidFromPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALID_FROM_PROPERTY_IRI_HTTP || *other == VALID_FROM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValidFromPropertyIri> for &str {
	fn eq(&self, other: &ValidFromPropertyIri) -> bool {
		*self == VALID_FROM_PROPERTY_IRI_HTTP || *self == VALID_FROM_PROPERTY_IRI_HTTPS
	}
}
pub struct ValidFromPropertyIriOrLabel;
impl PartialEq<&str> for ValidFromPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValidFromPropertyIri || *other == VALID_FROM_PROPERTY_LABEL
	}
}
impl PartialEq<ValidFromPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValidFromPropertyIriOrLabel) -> bool {
		*self == ValidFromPropertyIri || *self == VALID_FROM_PROPERTY_LABEL
	}
}
