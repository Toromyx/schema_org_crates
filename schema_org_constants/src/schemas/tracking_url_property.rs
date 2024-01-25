/// <https://schema.org/trackingUrl>
pub const TRACKING_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/trackingUrl";
/// <https://schema.org/trackingUrl>
pub const TRACKING_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/trackingUrl";
/// <https://schema.org/trackingUrl>
pub const TRACKING_URL_PROPERTY_LABEL: &str = "trackingUrl";
pub struct TrackingUrlPropertyIri;
impl PartialEq<&str> for TrackingUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRACKING_URL_PROPERTY_IRI_HTTP || *other == TRACKING_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TrackingUrlPropertyIri> for &str {
	fn eq(&self, other: &TrackingUrlPropertyIri) -> bool {
		*self == TRACKING_URL_PROPERTY_IRI_HTTP || *self == TRACKING_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct TrackingUrlPropertyIriOrLabel;
impl PartialEq<&str> for TrackingUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrackingUrlPropertyIri || *other == TRACKING_URL_PROPERTY_LABEL
	}
}
impl PartialEq<TrackingUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TrackingUrlPropertyIriOrLabel) -> bool {
		*self == TrackingUrlPropertyIri || *self == TRACKING_URL_PROPERTY_LABEL
	}
}
