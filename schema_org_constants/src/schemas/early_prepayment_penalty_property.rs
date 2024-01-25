/// <https://schema.org/earlyPrepaymentPenalty>
pub const EARLY_PREPAYMENT_PENALTY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/earlyPrepaymentPenalty";
/// <https://schema.org/earlyPrepaymentPenalty>
pub const EARLY_PREPAYMENT_PENALTY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/earlyPrepaymentPenalty";
/// <https://schema.org/earlyPrepaymentPenalty>
pub const EARLY_PREPAYMENT_PENALTY_PROPERTY_LABEL: &str = "earlyPrepaymentPenalty";
pub struct EarlyPrepaymentPenaltyPropertyIri;
impl PartialEq<&str> for EarlyPrepaymentPenaltyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EARLY_PREPAYMENT_PENALTY_PROPERTY_IRI_HTTP
			|| *other == EARLY_PREPAYMENT_PENALTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EarlyPrepaymentPenaltyPropertyIri> for &str {
	fn eq(&self, other: &EarlyPrepaymentPenaltyPropertyIri) -> bool {
		*self == EARLY_PREPAYMENT_PENALTY_PROPERTY_IRI_HTTP
			|| *self == EARLY_PREPAYMENT_PENALTY_PROPERTY_IRI_HTTPS
	}
}
pub struct EarlyPrepaymentPenaltyPropertyIriOrLabel;
impl PartialEq<&str> for EarlyPrepaymentPenaltyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EarlyPrepaymentPenaltyPropertyIri
			|| *other == EARLY_PREPAYMENT_PENALTY_PROPERTY_LABEL
	}
}
impl PartialEq<EarlyPrepaymentPenaltyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EarlyPrepaymentPenaltyPropertyIriOrLabel) -> bool {
		*self == EarlyPrepaymentPenaltyPropertyIri
			|| *self == EARLY_PREPAYMENT_PENALTY_PROPERTY_LABEL
	}
}
