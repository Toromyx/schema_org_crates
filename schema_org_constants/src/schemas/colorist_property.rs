/// <https://schema.org/colorist>
pub const COLORIST_PROPERTY_IRI_HTTP: &str = "http://schema.org/colorist";
/// <https://schema.org/colorist>
pub const COLORIST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/colorist";
/// <https://schema.org/colorist>
pub const COLORIST_PROPERTY_LABEL: &str = "colorist";
pub struct ColoristPropertyIri;
impl PartialEq<&str> for ColoristPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COLORIST_PROPERTY_IRI_HTTP || *other == COLORIST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ColoristPropertyIri> for &str {
	fn eq(&self, other: &ColoristPropertyIri) -> bool {
		*self == COLORIST_PROPERTY_IRI_HTTP || *self == COLORIST_PROPERTY_IRI_HTTPS
	}
}
pub struct ColoristPropertyIriOrLabel;
impl PartialEq<&str> for ColoristPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ColoristPropertyIri || *other == COLORIST_PROPERTY_LABEL
	}
}
impl PartialEq<ColoristPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ColoristPropertyIriOrLabel) -> bool {
		*self == ColoristPropertyIri || *self == COLORIST_PROPERTY_LABEL
	}
}
