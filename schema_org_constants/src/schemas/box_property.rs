/// <https://schema.org/box>
pub const BOX_PROPERTY_IRI_HTTP: &str = "http://schema.org/box";
/// <https://schema.org/box>
pub const BOX_PROPERTY_IRI_HTTPS: &str = "https://schema.org/box";
/// <https://schema.org/box>
pub const BOX_PROPERTY_LABEL: &str = "box";
pub struct BoxPropertyIri;
impl PartialEq<&str> for BoxPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOX_PROPERTY_IRI_HTTP || *other == BOX_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BoxPropertyIri> for &str {
	fn eq(&self, other: &BoxPropertyIri) -> bool {
		*self == BOX_PROPERTY_IRI_HTTP || *self == BOX_PROPERTY_IRI_HTTPS
	}
}
pub struct BoxPropertyIriOrLabel;
impl PartialEq<&str> for BoxPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BoxPropertyIri || *other == BOX_PROPERTY_LABEL
	}
}
impl PartialEq<BoxPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BoxPropertyIriOrLabel) -> bool {
		*self == BoxPropertyIri || *self == BOX_PROPERTY_LABEL
	}
}
