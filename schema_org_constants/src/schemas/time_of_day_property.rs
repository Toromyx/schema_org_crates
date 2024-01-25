/// <https://schema.org/timeOfDay>
pub const TIME_OF_DAY_PROPERTY_IRI_HTTP: &str = "http://schema.org/timeOfDay";
/// <https://schema.org/timeOfDay>
pub const TIME_OF_DAY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/timeOfDay";
/// <https://schema.org/timeOfDay>
pub const TIME_OF_DAY_PROPERTY_LABEL: &str = "timeOfDay";
pub struct TimeOfDayPropertyIri;
impl PartialEq<&str> for TimeOfDayPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TIME_OF_DAY_PROPERTY_IRI_HTTP || *other == TIME_OF_DAY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TimeOfDayPropertyIri> for &str {
	fn eq(&self, other: &TimeOfDayPropertyIri) -> bool {
		*self == TIME_OF_DAY_PROPERTY_IRI_HTTP || *self == TIME_OF_DAY_PROPERTY_IRI_HTTPS
	}
}
pub struct TimeOfDayPropertyIriOrLabel;
impl PartialEq<&str> for TimeOfDayPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TimeOfDayPropertyIri || *other == TIME_OF_DAY_PROPERTY_LABEL
	}
}
impl PartialEq<TimeOfDayPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TimeOfDayPropertyIriOrLabel) -> bool {
		*self == TimeOfDayPropertyIri || *self == TIME_OF_DAY_PROPERTY_LABEL
	}
}
