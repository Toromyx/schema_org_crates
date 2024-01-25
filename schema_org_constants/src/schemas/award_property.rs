/// <https://schema.org/award>
pub const AWARD_PROPERTY_IRI_HTTP: &str = "http://schema.org/award";
/// <https://schema.org/award>
pub const AWARD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/award";
/// <https://schema.org/award>
pub const AWARD_PROPERTY_LABEL: &str = "award";
pub struct AwardPropertyIri;
impl PartialEq<&str> for AwardPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AWARD_PROPERTY_IRI_HTTP || *other == AWARD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AwardPropertyIri> for &str {
	fn eq(&self, other: &AwardPropertyIri) -> bool {
		*self == AWARD_PROPERTY_IRI_HTTP || *self == AWARD_PROPERTY_IRI_HTTPS
	}
}
pub struct AwardPropertyIriOrLabel;
impl PartialEq<&str> for AwardPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AwardPropertyIri || *other == AWARD_PROPERTY_LABEL
	}
}
impl PartialEq<AwardPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AwardPropertyIriOrLabel) -> bool {
		*self == AwardPropertyIri || *self == AWARD_PROPERTY_LABEL
	}
}
