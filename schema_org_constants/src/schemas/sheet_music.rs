/// <https://schema.org/SheetMusic>
pub const SHEET_MUSIC_IRI_HTTP: &str = "http://schema.org/SheetMusic";
/// <https://schema.org/SheetMusic>
pub const SHEET_MUSIC_IRI_HTTPS: &str = "https://schema.org/SheetMusic";
/// <https://schema.org/SheetMusic>
pub const SHEET_MUSIC_LABEL: &str = "SheetMusic";
pub struct SheetMusicIri;
impl PartialEq<&str> for SheetMusicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHEET_MUSIC_IRI_HTTP || *other == SHEET_MUSIC_IRI_HTTPS
	}
}
impl PartialEq<SheetMusicIri> for &str {
	fn eq(&self, other: &SheetMusicIri) -> bool {
		*self == SHEET_MUSIC_IRI_HTTP || *self == SHEET_MUSIC_IRI_HTTPS
	}
}
pub struct SheetMusicIriOrLabel;
impl PartialEq<&str> for SheetMusicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SheetMusicIri || *other == SHEET_MUSIC_LABEL
	}
}
impl PartialEq<SheetMusicIriOrLabel> for &str {
	fn eq(&self, other: &SheetMusicIriOrLabel) -> bool {
		*self == SheetMusicIri || *self == SHEET_MUSIC_LABEL
	}
}
