/// <https://schema.org/webCheckinTime>
pub const WEB_CHECKIN_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/webCheckinTime";
/// <https://schema.org/webCheckinTime>
pub const WEB_CHECKIN_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/webCheckinTime";
/// <https://schema.org/webCheckinTime>
pub const WEB_CHECKIN_TIME_PROPERTY_LABEL: &str = "webCheckinTime";
pub struct WebCheckinTimePropertyIri;
impl PartialEq<&str> for WebCheckinTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEB_CHECKIN_TIME_PROPERTY_IRI_HTTP
			|| *other == WEB_CHECKIN_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WebCheckinTimePropertyIri> for &str {
	fn eq(&self, other: &WebCheckinTimePropertyIri) -> bool {
		*self == WEB_CHECKIN_TIME_PROPERTY_IRI_HTTP || *self == WEB_CHECKIN_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct WebCheckinTimePropertyIriOrLabel;
impl PartialEq<&str> for WebCheckinTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WebCheckinTimePropertyIri || *other == WEB_CHECKIN_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<WebCheckinTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &WebCheckinTimePropertyIriOrLabel) -> bool {
		*self == WebCheckinTimePropertyIri || *self == WEB_CHECKIN_TIME_PROPERTY_LABEL
	}
}
