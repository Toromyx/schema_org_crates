/// <https://schema.org/applicableLocation>
pub const APPLICABLE_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/applicableLocation";
/// <https://schema.org/applicableLocation>
pub const APPLICABLE_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/applicableLocation";
/// <https://schema.org/applicableLocation>
pub const APPLICABLE_LOCATION_PROPERTY_LABEL: &str = "applicableLocation";
pub struct ApplicableLocationPropertyIri;
impl PartialEq<&str> for ApplicableLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLICABLE_LOCATION_PROPERTY_IRI_HTTP
			|| *other == APPLICABLE_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ApplicableLocationPropertyIri> for &str {
	fn eq(&self, other: &ApplicableLocationPropertyIri) -> bool {
		*self == APPLICABLE_LOCATION_PROPERTY_IRI_HTTP
			|| *self == APPLICABLE_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ApplicableLocationPropertyIriOrLabel;
impl PartialEq<&str> for ApplicableLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplicableLocationPropertyIri || *other == APPLICABLE_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<ApplicableLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ApplicableLocationPropertyIriOrLabel) -> bool {
		*self == ApplicableLocationPropertyIri || *self == APPLICABLE_LOCATION_PROPERTY_LABEL
	}
}
