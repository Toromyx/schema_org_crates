/// <https://schema.org/colorSwatch>
pub const COLOR_SWATCH_PROPERTY_IRI_HTTP: &str = "http://schema.org/colorSwatch";
/// <https://schema.org/colorSwatch>
pub const COLOR_SWATCH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/colorSwatch";
/// <https://schema.org/colorSwatch>
pub const COLOR_SWATCH_PROPERTY_LABEL: &str = "colorSwatch";
pub struct ColorSwatchPropertyIri;
impl PartialEq<&str> for ColorSwatchPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COLOR_SWATCH_PROPERTY_IRI_HTTP || *other == COLOR_SWATCH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ColorSwatchPropertyIri> for &str {
	fn eq(&self, other: &ColorSwatchPropertyIri) -> bool {
		*self == COLOR_SWATCH_PROPERTY_IRI_HTTP || *self == COLOR_SWATCH_PROPERTY_IRI_HTTPS
	}
}
pub struct ColorSwatchPropertyIriOrLabel;
impl PartialEq<&str> for ColorSwatchPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ColorSwatchPropertyIri || *other == COLOR_SWATCH_PROPERTY_LABEL
	}
}
impl PartialEq<ColorSwatchPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ColorSwatchPropertyIriOrLabel) -> bool {
		*self == ColorSwatchPropertyIri || *self == COLOR_SWATCH_PROPERTY_LABEL
	}
}
