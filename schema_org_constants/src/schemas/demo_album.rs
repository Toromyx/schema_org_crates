/// <https://schema.org/DemoAlbum>
pub const DEMO_ALBUM_IRI_HTTP: &str = "http://schema.org/DemoAlbum";
/// <https://schema.org/DemoAlbum>
pub const DEMO_ALBUM_IRI_HTTPS: &str = "https://schema.org/DemoAlbum";
/// <https://schema.org/DemoAlbum>
pub const DEMO_ALBUM_LABEL: &str = "DemoAlbum";
pub struct DemoAlbumIri;
impl PartialEq<&str> for DemoAlbumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEMO_ALBUM_IRI_HTTP || *other == DEMO_ALBUM_IRI_HTTPS
	}
}
impl PartialEq<DemoAlbumIri> for &str {
	fn eq(&self, other: &DemoAlbumIri) -> bool {
		*self == DEMO_ALBUM_IRI_HTTP || *self == DEMO_ALBUM_IRI_HTTPS
	}
}
pub struct DemoAlbumIriOrLabel;
impl PartialEq<&str> for DemoAlbumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DemoAlbumIri || *other == DEMO_ALBUM_LABEL
	}
}
impl PartialEq<DemoAlbumIriOrLabel> for &str {
	fn eq(&self, other: &DemoAlbumIriOrLabel) -> bool {
		*self == DemoAlbumIri || *self == DEMO_ALBUM_LABEL
	}
}
