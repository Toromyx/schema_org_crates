/// <https://schema.org/albumProductionType>
pub const ALBUM_PRODUCTION_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/albumProductionType";
/// <https://schema.org/albumProductionType>
pub const ALBUM_PRODUCTION_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/albumProductionType";
/// <https://schema.org/albumProductionType>
pub const ALBUM_PRODUCTION_TYPE_PROPERTY_LABEL: &str = "albumProductionType";
pub struct AlbumProductionTypePropertyIri;
impl PartialEq<&str> for AlbumProductionTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALBUM_PRODUCTION_TYPE_PROPERTY_IRI_HTTP
			|| *other == ALBUM_PRODUCTION_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlbumProductionTypePropertyIri> for &str {
	fn eq(&self, other: &AlbumProductionTypePropertyIri) -> bool {
		*self == ALBUM_PRODUCTION_TYPE_PROPERTY_IRI_HTTP
			|| *self == ALBUM_PRODUCTION_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct AlbumProductionTypePropertyIriOrLabel;
impl PartialEq<&str> for AlbumProductionTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlbumProductionTypePropertyIri || *other == ALBUM_PRODUCTION_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<AlbumProductionTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlbumProductionTypePropertyIriOrLabel) -> bool {
		*self == AlbumProductionTypePropertyIri || *self == ALBUM_PRODUCTION_TYPE_PROPERTY_LABEL
	}
}
