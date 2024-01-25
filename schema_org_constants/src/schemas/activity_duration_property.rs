/// <https://schema.org/activityDuration>
pub const ACTIVITY_DURATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/activityDuration";
/// <https://schema.org/activityDuration>
pub const ACTIVITY_DURATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/activityDuration";
/// <https://schema.org/activityDuration>
pub const ACTIVITY_DURATION_PROPERTY_LABEL: &str = "activityDuration";
pub struct ActivityDurationPropertyIri;
impl PartialEq<&str> for ActivityDurationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTIVITY_DURATION_PROPERTY_IRI_HTTP
			|| *other == ACTIVITY_DURATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActivityDurationPropertyIri> for &str {
	fn eq(&self, other: &ActivityDurationPropertyIri) -> bool {
		*self == ACTIVITY_DURATION_PROPERTY_IRI_HTTP
			|| *self == ACTIVITY_DURATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ActivityDurationPropertyIriOrLabel;
impl PartialEq<&str> for ActivityDurationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActivityDurationPropertyIri || *other == ACTIVITY_DURATION_PROPERTY_LABEL
	}
}
impl PartialEq<ActivityDurationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActivityDurationPropertyIriOrLabel) -> bool {
		*self == ActivityDurationPropertyIri || *self == ACTIVITY_DURATION_PROPERTY_LABEL
	}
}
