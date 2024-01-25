/// <https://schema.org/Drawing>
pub const DRAWING_IRI_HTTP: &str = "http://schema.org/Drawing";
/// <https://schema.org/Drawing>
pub const DRAWING_IRI_HTTPS: &str = "https://schema.org/Drawing";
/// <https://schema.org/Drawing>
pub const DRAWING_LABEL: &str = "Drawing";
pub struct DrawingIri;
impl PartialEq<&str> for DrawingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRAWING_IRI_HTTP || *other == DRAWING_IRI_HTTPS
	}
}
impl PartialEq<DrawingIri> for &str {
	fn eq(&self, other: &DrawingIri) -> bool {
		*self == DRAWING_IRI_HTTP || *self == DRAWING_IRI_HTTPS
	}
}
pub struct DrawingIriOrLabel;
impl PartialEq<&str> for DrawingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrawingIri || *other == DRAWING_LABEL
	}
}
impl PartialEq<DrawingIriOrLabel> for &str {
	fn eq(&self, other: &DrawingIriOrLabel) -> bool {
		*self == DrawingIri || *self == DRAWING_LABEL
	}
}
