/// <https://schema.org/GraphicNovel>
pub const GRAPHIC_NOVEL_IRI_HTTP: &str = "http://schema.org/GraphicNovel";
/// <https://schema.org/GraphicNovel>
pub const GRAPHIC_NOVEL_IRI_HTTPS: &str = "https://schema.org/GraphicNovel";
/// <https://schema.org/GraphicNovel>
pub const GRAPHIC_NOVEL_LABEL: &str = "GraphicNovel";
pub struct GraphicNovelIri;
impl PartialEq<&str> for GraphicNovelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GRAPHIC_NOVEL_IRI_HTTP || *other == GRAPHIC_NOVEL_IRI_HTTPS
	}
}
impl PartialEq<GraphicNovelIri> for &str {
	fn eq(&self, other: &GraphicNovelIri) -> bool {
		*self == GRAPHIC_NOVEL_IRI_HTTP || *self == GRAPHIC_NOVEL_IRI_HTTPS
	}
}
pub struct GraphicNovelIriOrLabel;
impl PartialEq<&str> for GraphicNovelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GraphicNovelIri || *other == GRAPHIC_NOVEL_LABEL
	}
}
impl PartialEq<GraphicNovelIriOrLabel> for &str {
	fn eq(&self, other: &GraphicNovelIriOrLabel) -> bool {
		*self == GraphicNovelIri || *self == GRAPHIC_NOVEL_LABEL
	}
}
