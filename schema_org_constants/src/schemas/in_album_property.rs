/// <https://schema.org/inAlbum>
pub const IN_ALBUM_PROPERTY_IRI_HTTP: &str = "http://schema.org/inAlbum";
/// <https://schema.org/inAlbum>
pub const IN_ALBUM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inAlbum";
/// <https://schema.org/inAlbum>
pub const IN_ALBUM_PROPERTY_LABEL: &str = "inAlbum";
pub struct InAlbumPropertyIri;
impl PartialEq<&str> for InAlbumPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_ALBUM_PROPERTY_IRI_HTTP || *other == IN_ALBUM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InAlbumPropertyIri> for &str {
	fn eq(&self, other: &InAlbumPropertyIri) -> bool {
		*self == IN_ALBUM_PROPERTY_IRI_HTTP || *self == IN_ALBUM_PROPERTY_IRI_HTTPS
	}
}
pub struct InAlbumPropertyIriOrLabel;
impl PartialEq<&str> for InAlbumPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InAlbumPropertyIri || *other == IN_ALBUM_PROPERTY_LABEL
	}
}
impl PartialEq<InAlbumPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InAlbumPropertyIriOrLabel) -> bool {
		*self == InAlbumPropertyIri || *self == IN_ALBUM_PROPERTY_LABEL
	}
}
