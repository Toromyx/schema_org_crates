/// <https://schema.org/eligibleDuration>
pub const ELIGIBLE_DURATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/eligibleDuration";
/// <https://schema.org/eligibleDuration>
pub const ELIGIBLE_DURATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/eligibleDuration";
/// <https://schema.org/eligibleDuration>
pub const ELIGIBLE_DURATION_PROPERTY_LABEL: &str = "eligibleDuration";
pub struct EligibleDurationPropertyIri;
impl PartialEq<&str> for EligibleDurationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ELIGIBLE_DURATION_PROPERTY_IRI_HTTP
			|| *other == ELIGIBLE_DURATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EligibleDurationPropertyIri> for &str {
	fn eq(&self, other: &EligibleDurationPropertyIri) -> bool {
		*self == ELIGIBLE_DURATION_PROPERTY_IRI_HTTP
			|| *self == ELIGIBLE_DURATION_PROPERTY_IRI_HTTPS
	}
}
pub struct EligibleDurationPropertyIriOrLabel;
impl PartialEq<&str> for EligibleDurationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EligibleDurationPropertyIri || *other == ELIGIBLE_DURATION_PROPERTY_LABEL
	}
}
impl PartialEq<EligibleDurationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EligibleDurationPropertyIriOrLabel) -> bool {
		*self == EligibleDurationPropertyIri || *self == ELIGIBLE_DURATION_PROPERTY_LABEL
	}
}
