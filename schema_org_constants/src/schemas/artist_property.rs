/// <https://schema.org/artist>
pub const ARTIST_PROPERTY_IRI_HTTP: &str = "http://schema.org/artist";
/// <https://schema.org/artist>
pub const ARTIST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/artist";
/// <https://schema.org/artist>
pub const ARTIST_PROPERTY_LABEL: &str = "artist";
pub struct ArtistPropertyIri;
impl PartialEq<&str> for ArtistPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARTIST_PROPERTY_IRI_HTTP || *other == ARTIST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArtistPropertyIri> for &str {
	fn eq(&self, other: &ArtistPropertyIri) -> bool {
		*self == ARTIST_PROPERTY_IRI_HTTP || *self == ARTIST_PROPERTY_IRI_HTTPS
	}
}
pub struct ArtistPropertyIriOrLabel;
impl PartialEq<&str> for ArtistPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArtistPropertyIri || *other == ARTIST_PROPERTY_LABEL
	}
}
impl PartialEq<ArtistPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArtistPropertyIriOrLabel) -> bool {
		*self == ArtistPropertyIri || *self == ARTIST_PROPERTY_LABEL
	}
}
