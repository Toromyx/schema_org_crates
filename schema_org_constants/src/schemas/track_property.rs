/// <https://schema.org/track>
pub const TRACK_PROPERTY_IRI_HTTP: &str = "http://schema.org/track";
/// <https://schema.org/track>
pub const TRACK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/track";
/// <https://schema.org/track>
pub const TRACK_PROPERTY_LABEL: &str = "track";
pub struct TrackPropertyIri;
impl PartialEq<&str> for TrackPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRACK_PROPERTY_IRI_HTTP || *other == TRACK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TrackPropertyIri> for &str {
	fn eq(&self, other: &TrackPropertyIri) -> bool {
		*self == TRACK_PROPERTY_IRI_HTTP || *self == TRACK_PROPERTY_IRI_HTTPS
	}
}
pub struct TrackPropertyIriOrLabel;
impl PartialEq<&str> for TrackPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrackPropertyIri || *other == TRACK_PROPERTY_LABEL
	}
}
impl PartialEq<TrackPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TrackPropertyIriOrLabel) -> bool {
		*self == TrackPropertyIri || *self == TRACK_PROPERTY_LABEL
	}
}
