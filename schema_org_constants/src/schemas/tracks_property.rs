/// <https://schema.org/tracks>
#[deprecated = "This schema is superseded by <https://schema.org/track>."]
pub const TRACKS_PROPERTY_IRI_HTTP: &str = "http://schema.org/tracks";
/// <https://schema.org/tracks>
#[deprecated = "This schema is superseded by <https://schema.org/track>."]
pub const TRACKS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/tracks";
/// <https://schema.org/tracks>
#[deprecated = "This schema is superseded by <https://schema.org/track>."]
pub const TRACKS_PROPERTY_LABEL: &str = "tracks";
pub struct TracksPropertyIri;
impl PartialEq<&str> for TracksPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRACKS_PROPERTY_IRI_HTTP || *other == TRACKS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TracksPropertyIri> for &str {
	fn eq(&self, other: &TracksPropertyIri) -> bool {
		*self == TRACKS_PROPERTY_IRI_HTTP || *self == TRACKS_PROPERTY_IRI_HTTPS
	}
}
pub struct TracksPropertyIriOrLabel;
impl PartialEq<&str> for TracksPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TracksPropertyIri || *other == TRACKS_PROPERTY_LABEL
	}
}
impl PartialEq<TracksPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TracksPropertyIriOrLabel) -> bool {
		*self == TracksPropertyIri || *self == TRACKS_PROPERTY_LABEL
	}
}
