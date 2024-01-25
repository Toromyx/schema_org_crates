/// <https://schema.org/activityFrequency>
pub const ACTIVITY_FREQUENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/activityFrequency";
/// <https://schema.org/activityFrequency>
pub const ACTIVITY_FREQUENCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/activityFrequency";
/// <https://schema.org/activityFrequency>
pub const ACTIVITY_FREQUENCY_PROPERTY_LABEL: &str = "activityFrequency";
pub struct ActivityFrequencyPropertyIri;
impl PartialEq<&str> for ActivityFrequencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTIVITY_FREQUENCY_PROPERTY_IRI_HTTP
			|| *other == ACTIVITY_FREQUENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActivityFrequencyPropertyIri> for &str {
	fn eq(&self, other: &ActivityFrequencyPropertyIri) -> bool {
		*self == ACTIVITY_FREQUENCY_PROPERTY_IRI_HTTP
			|| *self == ACTIVITY_FREQUENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct ActivityFrequencyPropertyIriOrLabel;
impl PartialEq<&str> for ActivityFrequencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActivityFrequencyPropertyIri || *other == ACTIVITY_FREQUENCY_PROPERTY_LABEL
	}
}
impl PartialEq<ActivityFrequencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActivityFrequencyPropertyIriOrLabel) -> bool {
		*self == ActivityFrequencyPropertyIri || *self == ACTIVITY_FREQUENCY_PROPERTY_LABEL
	}
}
