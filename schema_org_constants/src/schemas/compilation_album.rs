/// <https://schema.org/CompilationAlbum>
pub const COMPILATION_ALBUM_IRI_HTTP: &str = "http://schema.org/CompilationAlbum";
/// <https://schema.org/CompilationAlbum>
pub const COMPILATION_ALBUM_IRI_HTTPS: &str = "https://schema.org/CompilationAlbum";
/// <https://schema.org/CompilationAlbum>
pub const COMPILATION_ALBUM_LABEL: &str = "CompilationAlbum";
pub struct CompilationAlbumIri;
impl PartialEq<&str> for CompilationAlbumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPILATION_ALBUM_IRI_HTTP || *other == COMPILATION_ALBUM_IRI_HTTPS
	}
}
impl PartialEq<CompilationAlbumIri> for &str {
	fn eq(&self, other: &CompilationAlbumIri) -> bool {
		*self == COMPILATION_ALBUM_IRI_HTTP || *self == COMPILATION_ALBUM_IRI_HTTPS
	}
}
pub struct CompilationAlbumIriOrLabel;
impl PartialEq<&str> for CompilationAlbumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CompilationAlbumIri || *other == COMPILATION_ALBUM_LABEL
	}
}
impl PartialEq<CompilationAlbumIriOrLabel> for &str {
	fn eq(&self, other: &CompilationAlbumIriOrLabel) -> bool {
		*self == CompilationAlbumIri || *self == COMPILATION_ALBUM_LABEL
	}
}
