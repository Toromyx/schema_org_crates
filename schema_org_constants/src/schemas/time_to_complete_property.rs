/// <https://schema.org/timeToComplete>
pub const TIME_TO_COMPLETE_PROPERTY_IRI_HTTP: &str = "http://schema.org/timeToComplete";
/// <https://schema.org/timeToComplete>
pub const TIME_TO_COMPLETE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/timeToComplete";
/// <https://schema.org/timeToComplete>
pub const TIME_TO_COMPLETE_PROPERTY_LABEL: &str = "timeToComplete";
pub struct TimeToCompletePropertyIri;
impl PartialEq<&str> for TimeToCompletePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TIME_TO_COMPLETE_PROPERTY_IRI_HTTP
			|| *other == TIME_TO_COMPLETE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TimeToCompletePropertyIri> for &str {
	fn eq(&self, other: &TimeToCompletePropertyIri) -> bool {
		*self == TIME_TO_COMPLETE_PROPERTY_IRI_HTTP || *self == TIME_TO_COMPLETE_PROPERTY_IRI_HTTPS
	}
}
pub struct TimeToCompletePropertyIriOrLabel;
impl PartialEq<&str> for TimeToCompletePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TimeToCompletePropertyIri || *other == TIME_TO_COMPLETE_PROPERTY_LABEL
	}
}
impl PartialEq<TimeToCompletePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TimeToCompletePropertyIriOrLabel) -> bool {
		*self == TimeToCompletePropertyIri || *self == TIME_TO_COMPLETE_PROPERTY_LABEL
	}
}
