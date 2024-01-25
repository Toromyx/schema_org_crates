/// <https://schema.org/LiveAlbum>
pub const LIVE_ALBUM_IRI_HTTP: &str = "http://schema.org/LiveAlbum";
/// <https://schema.org/LiveAlbum>
pub const LIVE_ALBUM_IRI_HTTPS: &str = "https://schema.org/LiveAlbum";
/// <https://schema.org/LiveAlbum>
pub const LIVE_ALBUM_LABEL: &str = "LiveAlbum";
pub struct LiveAlbumIri;
impl PartialEq<&str> for LiveAlbumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIVE_ALBUM_IRI_HTTP || *other == LIVE_ALBUM_IRI_HTTPS
	}
}
impl PartialEq<LiveAlbumIri> for &str {
	fn eq(&self, other: &LiveAlbumIri) -> bool {
		*self == LIVE_ALBUM_IRI_HTTP || *self == LIVE_ALBUM_IRI_HTTPS
	}
}
pub struct LiveAlbumIriOrLabel;
impl PartialEq<&str> for LiveAlbumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LiveAlbumIri || *other == LIVE_ALBUM_LABEL
	}
}
impl PartialEq<LiveAlbumIriOrLabel> for &str {
	fn eq(&self, other: &LiveAlbumIriOrLabel) -> bool {
		*self == LiveAlbumIri || *self == LIVE_ALBUM_LABEL
	}
}
