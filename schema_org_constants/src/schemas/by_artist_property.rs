/// <https://schema.org/byArtist>
pub const BY_ARTIST_PROPERTY_IRI_HTTP: &str = "http://schema.org/byArtist";
/// <https://schema.org/byArtist>
pub const BY_ARTIST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/byArtist";
/// <https://schema.org/byArtist>
pub const BY_ARTIST_PROPERTY_LABEL: &str = "byArtist";
pub struct ByArtistPropertyIri;
impl PartialEq<&str> for ByArtistPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BY_ARTIST_PROPERTY_IRI_HTTP || *other == BY_ARTIST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ByArtistPropertyIri> for &str {
	fn eq(&self, other: &ByArtistPropertyIri) -> bool {
		*self == BY_ARTIST_PROPERTY_IRI_HTTP || *self == BY_ARTIST_PROPERTY_IRI_HTTPS
	}
}
pub struct ByArtistPropertyIriOrLabel;
impl PartialEq<&str> for ByArtistPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ByArtistPropertyIri || *other == BY_ARTIST_PROPERTY_LABEL
	}
}
impl PartialEq<ByArtistPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ByArtistPropertyIriOrLabel) -> bool {
		*self == ByArtistPropertyIri || *self == BY_ARTIST_PROPERTY_LABEL
	}
}
