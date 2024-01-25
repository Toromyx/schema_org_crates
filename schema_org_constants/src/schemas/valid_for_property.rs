/// <https://schema.org/validFor>
pub const VALID_FOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/validFor";
/// <https://schema.org/validFor>
pub const VALID_FOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/validFor";
/// <https://schema.org/validFor>
pub const VALID_FOR_PROPERTY_LABEL: &str = "validFor";
pub struct ValidForPropertyIri;
impl PartialEq<&str> for ValidForPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALID_FOR_PROPERTY_IRI_HTTP || *other == VALID_FOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValidForPropertyIri> for &str {
	fn eq(&self, other: &ValidForPropertyIri) -> bool {
		*self == VALID_FOR_PROPERTY_IRI_HTTP || *self == VALID_FOR_PROPERTY_IRI_HTTPS
	}
}
pub struct ValidForPropertyIriOrLabel;
impl PartialEq<&str> for ValidForPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValidForPropertyIri || *other == VALID_FOR_PROPERTY_LABEL
	}
}
impl PartialEq<ValidForPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValidForPropertyIriOrLabel) -> bool {
		*self == ValidForPropertyIri || *self == VALID_FOR_PROPERTY_LABEL
	}
}
