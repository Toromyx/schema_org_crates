/// <https://schema.org/scheduledTime>
pub const SCHEDULED_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/scheduledTime";
/// <https://schema.org/scheduledTime>
pub const SCHEDULED_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/scheduledTime";
/// <https://schema.org/scheduledTime>
pub const SCHEDULED_TIME_PROPERTY_LABEL: &str = "scheduledTime";
pub struct ScheduledTimePropertyIri;
impl PartialEq<&str> for ScheduledTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCHEDULED_TIME_PROPERTY_IRI_HTTP || *other == SCHEDULED_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ScheduledTimePropertyIri> for &str {
	fn eq(&self, other: &ScheduledTimePropertyIri) -> bool {
		*self == SCHEDULED_TIME_PROPERTY_IRI_HTTP || *self == SCHEDULED_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct ScheduledTimePropertyIriOrLabel;
impl PartialEq<&str> for ScheduledTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ScheduledTimePropertyIri || *other == SCHEDULED_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<ScheduledTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ScheduledTimePropertyIriOrLabel) -> bool {
		*self == ScheduledTimePropertyIri || *self == SCHEDULED_TIME_PROPERTY_LABEL
	}
}
