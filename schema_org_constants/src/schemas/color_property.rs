/// <https://schema.org/color>
pub const COLOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/color";
/// <https://schema.org/color>
pub const COLOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/color";
/// <https://schema.org/color>
pub const COLOR_PROPERTY_LABEL: &str = "color";
pub struct ColorPropertyIri;
impl PartialEq<&str> for ColorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COLOR_PROPERTY_IRI_HTTP || *other == COLOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ColorPropertyIri> for &str {
	fn eq(&self, other: &ColorPropertyIri) -> bool {
		*self == COLOR_PROPERTY_IRI_HTTP || *self == COLOR_PROPERTY_IRI_HTTPS
	}
}
pub struct ColorPropertyIriOrLabel;
impl PartialEq<&str> for ColorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ColorPropertyIri || *other == COLOR_PROPERTY_LABEL
	}
}
impl PartialEq<ColorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ColorPropertyIriOrLabel) -> bool {
		*self == ColorPropertyIri || *self == COLOR_PROPERTY_LABEL
	}
}
