/// <https://schema.org/maximumEnrollment>
pub const MAXIMUM_ENROLLMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/maximumEnrollment";
/// <https://schema.org/maximumEnrollment>
pub const MAXIMUM_ENROLLMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/maximumEnrollment";
/// <https://schema.org/maximumEnrollment>
pub const MAXIMUM_ENROLLMENT_PROPERTY_LABEL: &str = "maximumEnrollment";
pub struct MaximumEnrollmentPropertyIri;
impl PartialEq<&str> for MaximumEnrollmentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAXIMUM_ENROLLMENT_PROPERTY_IRI_HTTP
			|| *other == MAXIMUM_ENROLLMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MaximumEnrollmentPropertyIri> for &str {
	fn eq(&self, other: &MaximumEnrollmentPropertyIri) -> bool {
		*self == MAXIMUM_ENROLLMENT_PROPERTY_IRI_HTTP
			|| *self == MAXIMUM_ENROLLMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct MaximumEnrollmentPropertyIriOrLabel;
impl PartialEq<&str> for MaximumEnrollmentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaximumEnrollmentPropertyIri || *other == MAXIMUM_ENROLLMENT_PROPERTY_LABEL
	}
}
impl PartialEq<MaximumEnrollmentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MaximumEnrollmentPropertyIriOrLabel) -> bool {
		*self == MaximumEnrollmentPropertyIri || *self == MAXIMUM_ENROLLMENT_PROPERTY_LABEL
	}
}
