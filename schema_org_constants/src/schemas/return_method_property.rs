/// <https://schema.org/returnMethod>
pub const RETURN_METHOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/returnMethod";
/// <https://schema.org/returnMethod>
pub const RETURN_METHOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/returnMethod";
/// <https://schema.org/returnMethod>
pub const RETURN_METHOD_PROPERTY_LABEL: &str = "returnMethod";
pub struct ReturnMethodPropertyIri;
impl PartialEq<&str> for ReturnMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_METHOD_PROPERTY_IRI_HTTP || *other == RETURN_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReturnMethodPropertyIri> for &str {
	fn eq(&self, other: &ReturnMethodPropertyIri) -> bool {
		*self == RETURN_METHOD_PROPERTY_IRI_HTTP || *self == RETURN_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct ReturnMethodPropertyIriOrLabel;
impl PartialEq<&str> for ReturnMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnMethodPropertyIri || *other == RETURN_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<ReturnMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReturnMethodPropertyIriOrLabel) -> bool {
		*self == ReturnMethodPropertyIri || *self == RETURN_METHOD_PROPERTY_LABEL
	}
}
