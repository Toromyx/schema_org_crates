/// <https://schema.org/timeRequired>
pub const TIME_REQUIRED_PROPERTY_IRI_HTTP: &str = "http://schema.org/timeRequired";
/// <https://schema.org/timeRequired>
pub const TIME_REQUIRED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/timeRequired";
/// <https://schema.org/timeRequired>
pub const TIME_REQUIRED_PROPERTY_LABEL: &str = "timeRequired";
pub struct TimeRequiredPropertyIri;
impl PartialEq<&str> for TimeRequiredPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TIME_REQUIRED_PROPERTY_IRI_HTTP || *other == TIME_REQUIRED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TimeRequiredPropertyIri> for &str {
	fn eq(&self, other: &TimeRequiredPropertyIri) -> bool {
		*self == TIME_REQUIRED_PROPERTY_IRI_HTTP || *self == TIME_REQUIRED_PROPERTY_IRI_HTTPS
	}
}
pub struct TimeRequiredPropertyIriOrLabel;
impl PartialEq<&str> for TimeRequiredPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TimeRequiredPropertyIri || *other == TIME_REQUIRED_PROPERTY_LABEL
	}
}
impl PartialEq<TimeRequiredPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TimeRequiredPropertyIriOrLabel) -> bool {
		*self == TimeRequiredPropertyIri || *self == TIME_REQUIRED_PROPERTY_LABEL
	}
}
