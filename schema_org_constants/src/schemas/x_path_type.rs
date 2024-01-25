/// <https://schema.org/XPathType>
pub const X_PATH_TYPE_IRI_HTTP: &str = "http://schema.org/XPathType";
/// <https://schema.org/XPathType>
pub const X_PATH_TYPE_IRI_HTTPS: &str = "https://schema.org/XPathType";
/// <https://schema.org/XPathType>
pub const X_PATH_TYPE_LABEL: &str = "XPathType";
pub struct XPathTypeIri;
impl PartialEq<&str> for XPathTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == X_PATH_TYPE_IRI_HTTP || *other == X_PATH_TYPE_IRI_HTTPS
	}
}
impl PartialEq<XPathTypeIri> for &str {
	fn eq(&self, other: &XPathTypeIri) -> bool {
		*self == X_PATH_TYPE_IRI_HTTP || *self == X_PATH_TYPE_IRI_HTTPS
	}
}
pub struct XPathTypeIriOrLabel;
impl PartialEq<&str> for XPathTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == XPathTypeIri || *other == X_PATH_TYPE_LABEL
	}
}
impl PartialEq<XPathTypeIriOrLabel> for &str {
	fn eq(&self, other: &XPathTypeIriOrLabel) -> bool {
		*self == XPathTypeIri || *self == X_PATH_TYPE_LABEL
	}
}
