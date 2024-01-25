/// <https://schema.org/checkinTime>
pub const CHECKIN_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/checkinTime";
/// <https://schema.org/checkinTime>
pub const CHECKIN_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/checkinTime";
/// <https://schema.org/checkinTime>
pub const CHECKIN_TIME_PROPERTY_LABEL: &str = "checkinTime";
pub struct CheckinTimePropertyIri;
impl PartialEq<&str> for CheckinTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHECKIN_TIME_PROPERTY_IRI_HTTP || *other == CHECKIN_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CheckinTimePropertyIri> for &str {
	fn eq(&self, other: &CheckinTimePropertyIri) -> bool {
		*self == CHECKIN_TIME_PROPERTY_IRI_HTTP || *self == CHECKIN_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct CheckinTimePropertyIriOrLabel;
impl PartialEq<&str> for CheckinTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CheckinTimePropertyIri || *other == CHECKIN_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<CheckinTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CheckinTimePropertyIriOrLabel) -> bool {
		*self == CheckinTimePropertyIri || *self == CHECKIN_TIME_PROPERTY_LABEL
	}
}
