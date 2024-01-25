/// <https://schema.org/isResizable>
pub const IS_RESIZABLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/isResizable";
/// <https://schema.org/isResizable>
pub const IS_RESIZABLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isResizable";
/// <https://schema.org/isResizable>
pub const IS_RESIZABLE_PROPERTY_LABEL: &str = "isResizable";
pub struct IsResizablePropertyIri;
impl PartialEq<&str> for IsResizablePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_RESIZABLE_PROPERTY_IRI_HTTP || *other == IS_RESIZABLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsResizablePropertyIri> for &str {
	fn eq(&self, other: &IsResizablePropertyIri) -> bool {
		*self == IS_RESIZABLE_PROPERTY_IRI_HTTP || *self == IS_RESIZABLE_PROPERTY_IRI_HTTPS
	}
}
pub struct IsResizablePropertyIriOrLabel;
impl PartialEq<&str> for IsResizablePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsResizablePropertyIri || *other == IS_RESIZABLE_PROPERTY_LABEL
	}
}
impl PartialEq<IsResizablePropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsResizablePropertyIriOrLabel) -> bool {
		*self == IsResizablePropertyIri || *self == IS_RESIZABLE_PROPERTY_LABEL
	}
}
