/// <https://schema.org/intensity>
pub const INTENSITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/intensity";
/// <https://schema.org/intensity>
pub const INTENSITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/intensity";
/// <https://schema.org/intensity>
pub const INTENSITY_PROPERTY_LABEL: &str = "intensity";
pub struct IntensityPropertyIri;
impl PartialEq<&str> for IntensityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTENSITY_PROPERTY_IRI_HTTP || *other == INTENSITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IntensityPropertyIri> for &str {
	fn eq(&self, other: &IntensityPropertyIri) -> bool {
		*self == INTENSITY_PROPERTY_IRI_HTTP || *self == INTENSITY_PROPERTY_IRI_HTTPS
	}
}
pub struct IntensityPropertyIriOrLabel;
impl PartialEq<&str> for IntensityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IntensityPropertyIri || *other == INTENSITY_PROPERTY_LABEL
	}
}
impl PartialEq<IntensityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IntensityPropertyIriOrLabel) -> bool {
		*self == IntensityPropertyIri || *self == INTENSITY_PROPERTY_LABEL
	}
}
