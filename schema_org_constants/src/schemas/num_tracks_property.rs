/// <https://schema.org/numTracks>
pub const NUM_TRACKS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numTracks";
/// <https://schema.org/numTracks>
pub const NUM_TRACKS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numTracks";
/// <https://schema.org/numTracks>
pub const NUM_TRACKS_PROPERTY_LABEL: &str = "numTracks";
pub struct NumTracksPropertyIri;
impl PartialEq<&str> for NumTracksPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUM_TRACKS_PROPERTY_IRI_HTTP || *other == NUM_TRACKS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumTracksPropertyIri> for &str {
	fn eq(&self, other: &NumTracksPropertyIri) -> bool {
		*self == NUM_TRACKS_PROPERTY_IRI_HTTP || *self == NUM_TRACKS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumTracksPropertyIriOrLabel;
impl PartialEq<&str> for NumTracksPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumTracksPropertyIri || *other == NUM_TRACKS_PROPERTY_LABEL
	}
}
impl PartialEq<NumTracksPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumTracksPropertyIriOrLabel) -> bool {
		*self == NumTracksPropertyIri || *self == NUM_TRACKS_PROPERTY_LABEL
	}
}
