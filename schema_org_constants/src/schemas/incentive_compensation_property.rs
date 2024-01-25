/// <https://schema.org/incentiveCompensation>
pub const INCENTIVE_COMPENSATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/incentiveCompensation";
/// <https://schema.org/incentiveCompensation>
pub const INCENTIVE_COMPENSATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/incentiveCompensation";
/// <https://schema.org/incentiveCompensation>
pub const INCENTIVE_COMPENSATION_PROPERTY_LABEL: &str = "incentiveCompensation";
pub struct IncentiveCompensationPropertyIri;
impl PartialEq<&str> for IncentiveCompensationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCENTIVE_COMPENSATION_PROPERTY_IRI_HTTP
			|| *other == INCENTIVE_COMPENSATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncentiveCompensationPropertyIri> for &str {
	fn eq(&self, other: &IncentiveCompensationPropertyIri) -> bool {
		*self == INCENTIVE_COMPENSATION_PROPERTY_IRI_HTTP
			|| *self == INCENTIVE_COMPENSATION_PROPERTY_IRI_HTTPS
	}
}
pub struct IncentiveCompensationPropertyIriOrLabel;
impl PartialEq<&str> for IncentiveCompensationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncentiveCompensationPropertyIri
			|| *other == INCENTIVE_COMPENSATION_PROPERTY_LABEL
	}
}
impl PartialEq<IncentiveCompensationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncentiveCompensationPropertyIriOrLabel) -> bool {
		*self == IncentiveCompensationPropertyIri || *self == INCENTIVE_COMPENSATION_PROPERTY_LABEL
	}
}
