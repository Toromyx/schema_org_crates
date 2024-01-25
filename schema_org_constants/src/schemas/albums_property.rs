/// <https://schema.org/albums>
#[deprecated = "This schema is superseded by <https://schema.org/album>."]
pub const ALBUMS_PROPERTY_IRI_HTTP: &str = "http://schema.org/albums";
/// <https://schema.org/albums>
#[deprecated = "This schema is superseded by <https://schema.org/album>."]
pub const ALBUMS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/albums";
/// <https://schema.org/albums>
#[deprecated = "This schema is superseded by <https://schema.org/album>."]
pub const ALBUMS_PROPERTY_LABEL: &str = "albums";
pub struct AlbumsPropertyIri;
impl PartialEq<&str> for AlbumsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALBUMS_PROPERTY_IRI_HTTP || *other == ALBUMS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlbumsPropertyIri> for &str {
	fn eq(&self, other: &AlbumsPropertyIri) -> bool {
		*self == ALBUMS_PROPERTY_IRI_HTTP || *self == ALBUMS_PROPERTY_IRI_HTTPS
	}
}
pub struct AlbumsPropertyIriOrLabel;
impl PartialEq<&str> for AlbumsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlbumsPropertyIri || *other == ALBUMS_PROPERTY_LABEL
	}
}
impl PartialEq<AlbumsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlbumsPropertyIriOrLabel) -> bool {
		*self == AlbumsPropertyIri || *self == ALBUMS_PROPERTY_LABEL
	}
}
