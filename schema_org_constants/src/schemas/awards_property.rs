/// <https://schema.org/awards>
#[deprecated = "This schema is superseded by <https://schema.org/award>."]
pub const AWARDS_PROPERTY_IRI_HTTP: &str = "http://schema.org/awards";
/// <https://schema.org/awards>
#[deprecated = "This schema is superseded by <https://schema.org/award>."]
pub const AWARDS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/awards";
/// <https://schema.org/awards>
#[deprecated = "This schema is superseded by <https://schema.org/award>."]
pub const AWARDS_PROPERTY_LABEL: &str = "awards";
pub struct AwardsPropertyIri;
impl PartialEq<&str> for AwardsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AWARDS_PROPERTY_IRI_HTTP || *other == AWARDS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AwardsPropertyIri> for &str {
	fn eq(&self, other: &AwardsPropertyIri) -> bool {
		*self == AWARDS_PROPERTY_IRI_HTTP || *self == AWARDS_PROPERTY_IRI_HTTPS
	}
}
pub struct AwardsPropertyIriOrLabel;
impl PartialEq<&str> for AwardsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AwardsPropertyIri || *other == AWARDS_PROPERTY_LABEL
	}
}
impl PartialEq<AwardsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AwardsPropertyIriOrLabel) -> bool {
		*self == AwardsPropertyIri || *self == AWARDS_PROPERTY_LABEL
	}
}
