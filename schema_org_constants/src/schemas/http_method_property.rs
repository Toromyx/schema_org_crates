/// <https://schema.org/httpMethod>
pub const HTTP_METHOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/httpMethod";
/// <https://schema.org/httpMethod>
pub const HTTP_METHOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/httpMethod";
/// <https://schema.org/httpMethod>
pub const HTTP_METHOD_PROPERTY_LABEL: &str = "httpMethod";
pub struct HttpMethodPropertyIri;
impl PartialEq<&str> for HttpMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HTTP_METHOD_PROPERTY_IRI_HTTP || *other == HTTP_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HttpMethodPropertyIri> for &str {
	fn eq(&self, other: &HttpMethodPropertyIri) -> bool {
		*self == HTTP_METHOD_PROPERTY_IRI_HTTP || *self == HTTP_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct HttpMethodPropertyIriOrLabel;
impl PartialEq<&str> for HttpMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HttpMethodPropertyIri || *other == HTTP_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<HttpMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HttpMethodPropertyIriOrLabel) -> bool {
		*self == HttpMethodPropertyIri || *self == HTTP_METHOD_PROPERTY_LABEL
	}
}
