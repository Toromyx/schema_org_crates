/// <https://schema.org/totalHistoricalEnrollment>
pub const TOTAL_HISTORICAL_ENROLLMENT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/totalHistoricalEnrollment";
/// <https://schema.org/totalHistoricalEnrollment>
pub const TOTAL_HISTORICAL_ENROLLMENT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/totalHistoricalEnrollment";
/// <https://schema.org/totalHistoricalEnrollment>
pub const TOTAL_HISTORICAL_ENROLLMENT_PROPERTY_LABEL: &str = "totalHistoricalEnrollment";
pub struct TotalHistoricalEnrollmentPropertyIri;
impl PartialEq<&str> for TotalHistoricalEnrollmentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOTAL_HISTORICAL_ENROLLMENT_PROPERTY_IRI_HTTP
			|| *other == TOTAL_HISTORICAL_ENROLLMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TotalHistoricalEnrollmentPropertyIri> for &str {
	fn eq(&self, other: &TotalHistoricalEnrollmentPropertyIri) -> bool {
		*self == TOTAL_HISTORICAL_ENROLLMENT_PROPERTY_IRI_HTTP
			|| *self == TOTAL_HISTORICAL_ENROLLMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct TotalHistoricalEnrollmentPropertyIriOrLabel;
impl PartialEq<&str> for TotalHistoricalEnrollmentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TotalHistoricalEnrollmentPropertyIri
			|| *other == TOTAL_HISTORICAL_ENROLLMENT_PROPERTY_LABEL
	}
}
impl PartialEq<TotalHistoricalEnrollmentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TotalHistoricalEnrollmentPropertyIriOrLabel) -> bool {
		*self == TotalHistoricalEnrollmentPropertyIri
			|| *self == TOTAL_HISTORICAL_ENROLLMENT_PROPERTY_LABEL
	}
}
