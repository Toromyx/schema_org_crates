/// <https://schema.org/height>
pub const HEIGHT_PROPERTY_IRI_HTTP: &str = "http://schema.org/height";
/// <https://schema.org/height>
pub const HEIGHT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/height";
/// <https://schema.org/height>
pub const HEIGHT_PROPERTY_LABEL: &str = "height";
pub struct HeightPropertyIri;
impl PartialEq<&str> for HeightPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEIGHT_PROPERTY_IRI_HTTP || *other == HEIGHT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HeightPropertyIri> for &str {
	fn eq(&self, other: &HeightPropertyIri) -> bool {
		*self == HEIGHT_PROPERTY_IRI_HTTP || *self == HEIGHT_PROPERTY_IRI_HTTPS
	}
}
pub struct HeightPropertyIriOrLabel;
impl PartialEq<&str> for HeightPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HeightPropertyIri || *other == HEIGHT_PROPERTY_LABEL
	}
}
impl PartialEq<HeightPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HeightPropertyIriOrLabel) -> bool {
		*self == HeightPropertyIri || *self == HEIGHT_PROPERTY_LABEL
	}
}
