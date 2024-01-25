/// <https://schema.org/MusicAlbumProductionType>
pub const MUSIC_ALBUM_PRODUCTION_TYPE_IRI_HTTP: &str = "http://schema.org/MusicAlbumProductionType";
/// <https://schema.org/MusicAlbumProductionType>
pub const MUSIC_ALBUM_PRODUCTION_TYPE_IRI_HTTPS: &str =
	"https://schema.org/MusicAlbumProductionType";
/// <https://schema.org/MusicAlbumProductionType>
pub const MUSIC_ALBUM_PRODUCTION_TYPE_LABEL: &str = "MusicAlbumProductionType";
pub struct MusicAlbumProductionTypeIri;
impl PartialEq<&str> for MusicAlbumProductionTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_ALBUM_PRODUCTION_TYPE_IRI_HTTP
			|| *other == MUSIC_ALBUM_PRODUCTION_TYPE_IRI_HTTPS
	}
}
impl PartialEq<MusicAlbumProductionTypeIri> for &str {
	fn eq(&self, other: &MusicAlbumProductionTypeIri) -> bool {
		*self == MUSIC_ALBUM_PRODUCTION_TYPE_IRI_HTTP
			|| *self == MUSIC_ALBUM_PRODUCTION_TYPE_IRI_HTTPS
	}
}
pub struct MusicAlbumProductionTypeIriOrLabel;
impl PartialEq<&str> for MusicAlbumProductionTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicAlbumProductionTypeIri || *other == MUSIC_ALBUM_PRODUCTION_TYPE_LABEL
	}
}
impl PartialEq<MusicAlbumProductionTypeIriOrLabel> for &str {
	fn eq(&self, other: &MusicAlbumProductionTypeIriOrLabel) -> bool {
		*self == MusicAlbumProductionTypeIri || *self == MUSIC_ALBUM_PRODUCTION_TYPE_LABEL
	}
}
