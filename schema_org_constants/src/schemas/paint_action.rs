/// <https://schema.org/PaintAction>
pub const PAINT_ACTION_IRI_HTTP: &str = "http://schema.org/PaintAction";
/// <https://schema.org/PaintAction>
pub const PAINT_ACTION_IRI_HTTPS: &str = "https://schema.org/PaintAction";
/// <https://schema.org/PaintAction>
pub const PAINT_ACTION_LABEL: &str = "PaintAction";
pub struct PaintActionIri;
impl PartialEq<&str> for PaintActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAINT_ACTION_IRI_HTTP || *other == PAINT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<PaintActionIri> for &str {
	fn eq(&self, other: &PaintActionIri) -> bool {
		*self == PAINT_ACTION_IRI_HTTP || *self == PAINT_ACTION_IRI_HTTPS
	}
}
pub struct PaintActionIriOrLabel;
impl PartialEq<&str> for PaintActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaintActionIri || *other == PAINT_ACTION_LABEL
	}
}
impl PartialEq<PaintActionIriOrLabel> for &str {
	fn eq(&self, other: &PaintActionIriOrLabel) -> bool {
		*self == PaintActionIri || *self == PAINT_ACTION_LABEL
	}
}
