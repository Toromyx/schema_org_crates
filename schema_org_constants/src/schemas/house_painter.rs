/// <https://schema.org/HousePainter>
pub const HOUSE_PAINTER_IRI_HTTP: &str = "http://schema.org/HousePainter";
/// <https://schema.org/HousePainter>
pub const HOUSE_PAINTER_IRI_HTTPS: &str = "https://schema.org/HousePainter";
/// <https://schema.org/HousePainter>
pub const HOUSE_PAINTER_LABEL: &str = "HousePainter";
pub struct HousePainterIri;
impl PartialEq<&str> for HousePainterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOUSE_PAINTER_IRI_HTTP || *other == HOUSE_PAINTER_IRI_HTTPS
	}
}
impl PartialEq<HousePainterIri> for &str {
	fn eq(&self, other: &HousePainterIri) -> bool {
		*self == HOUSE_PAINTER_IRI_HTTP || *self == HOUSE_PAINTER_IRI_HTTPS
	}
}
pub struct HousePainterIriOrLabel;
impl PartialEq<&str> for HousePainterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HousePainterIri || *other == HOUSE_PAINTER_LABEL
	}
}
impl PartialEq<HousePainterIriOrLabel> for &str {
	fn eq(&self, other: &HousePainterIriOrLabel) -> bool {
		*self == HousePainterIri || *self == HOUSE_PAINTER_LABEL
	}
}
