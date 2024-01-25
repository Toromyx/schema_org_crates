/// <https://schema.org/width>
pub const WIDTH_PROPERTY_IRI_HTTP: &str = "http://schema.org/width";
/// <https://schema.org/width>
pub const WIDTH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/width";
/// <https://schema.org/width>
pub const WIDTH_PROPERTY_LABEL: &str = "width";
pub struct WidthPropertyIri;
impl PartialEq<&str> for WidthPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WIDTH_PROPERTY_IRI_HTTP || *other == WIDTH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WidthPropertyIri> for &str {
	fn eq(&self, other: &WidthPropertyIri) -> bool {
		*self == WIDTH_PROPERTY_IRI_HTTP || *self == WIDTH_PROPERTY_IRI_HTTPS
	}
}
pub struct WidthPropertyIriOrLabel;
impl PartialEq<&str> for WidthPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WidthPropertyIri || *other == WIDTH_PROPERTY_LABEL
	}
}
impl PartialEq<WidthPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WidthPropertyIriOrLabel) -> bool {
		*self == WidthPropertyIri || *self == WIDTH_PROPERTY_LABEL
	}
}
