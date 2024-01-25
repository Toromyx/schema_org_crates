/// <https://schema.org/MusicVenue>
pub const MUSIC_VENUE_IRI_HTTP: &str = "http://schema.org/MusicVenue";
/// <https://schema.org/MusicVenue>
pub const MUSIC_VENUE_IRI_HTTPS: &str = "https://schema.org/MusicVenue";
/// <https://schema.org/MusicVenue>
pub const MUSIC_VENUE_LABEL: &str = "MusicVenue";
pub struct MusicVenueIri;
impl PartialEq<&str> for MusicVenueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_VENUE_IRI_HTTP || *other == MUSIC_VENUE_IRI_HTTPS
	}
}
impl PartialEq<MusicVenueIri> for &str {
	fn eq(&self, other: &MusicVenueIri) -> bool {
		*self == MUSIC_VENUE_IRI_HTTP || *self == MUSIC_VENUE_IRI_HTTPS
	}
}
pub struct MusicVenueIriOrLabel;
impl PartialEq<&str> for MusicVenueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicVenueIri || *other == MUSIC_VENUE_LABEL
	}
}
impl PartialEq<MusicVenueIriOrLabel> for &str {
	fn eq(&self, other: &MusicVenueIriOrLabel) -> bool {
		*self == MusicVenueIri || *self == MUSIC_VENUE_LABEL
	}
}
