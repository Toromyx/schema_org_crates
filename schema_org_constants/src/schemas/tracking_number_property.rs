/// <https://schema.org/trackingNumber>
pub const TRACKING_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/trackingNumber";
/// <https://schema.org/trackingNumber>
pub const TRACKING_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/trackingNumber";
/// <https://schema.org/trackingNumber>
pub const TRACKING_NUMBER_PROPERTY_LABEL: &str = "trackingNumber";
pub struct TrackingNumberPropertyIri;
impl PartialEq<&str> for TrackingNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRACKING_NUMBER_PROPERTY_IRI_HTTP || *other == TRACKING_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TrackingNumberPropertyIri> for &str {
	fn eq(&self, other: &TrackingNumberPropertyIri) -> bool {
		*self == TRACKING_NUMBER_PROPERTY_IRI_HTTP || *self == TRACKING_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct TrackingNumberPropertyIriOrLabel;
impl PartialEq<&str> for TrackingNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrackingNumberPropertyIri || *other == TRACKING_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<TrackingNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TrackingNumberPropertyIriOrLabel) -> bool {
		*self == TrackingNumberPropertyIri || *self == TRACKING_NUMBER_PROPERTY_LABEL
	}
}
