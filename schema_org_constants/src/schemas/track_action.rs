/// <https://schema.org/TrackAction>
pub const TRACK_ACTION_IRI_HTTP: &str = "http://schema.org/TrackAction";
/// <https://schema.org/TrackAction>
pub const TRACK_ACTION_IRI_HTTPS: &str = "https://schema.org/TrackAction";
/// <https://schema.org/TrackAction>
pub const TRACK_ACTION_LABEL: &str = "TrackAction";
pub struct TrackActionIri;
impl PartialEq<&str> for TrackActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRACK_ACTION_IRI_HTTP || *other == TRACK_ACTION_IRI_HTTPS
	}
}
impl PartialEq<TrackActionIri> for &str {
	fn eq(&self, other: &TrackActionIri) -> bool {
		*self == TRACK_ACTION_IRI_HTTP || *self == TRACK_ACTION_IRI_HTTPS
	}
}
pub struct TrackActionIriOrLabel;
impl PartialEq<&str> for TrackActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrackActionIri || *other == TRACK_ACTION_LABEL
	}
}
impl PartialEq<TrackActionIriOrLabel> for &str {
	fn eq(&self, other: &TrackActionIriOrLabel) -> bool {
		*self == TrackActionIri || *self == TRACK_ACTION_LABEL
	}
}
