/// <https://schema.org/result>
pub const RESULT_PROPERTY_IRI_HTTP: &str = "http://schema.org/result";
/// <https://schema.org/result>
pub const RESULT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/result";
/// <https://schema.org/result>
pub const RESULT_PROPERTY_LABEL: &str = "result";
pub struct ResultPropertyIri;
impl PartialEq<&str> for ResultPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESULT_PROPERTY_IRI_HTTP || *other == RESULT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ResultPropertyIri> for &str {
	fn eq(&self, other: &ResultPropertyIri) -> bool {
		*self == RESULT_PROPERTY_IRI_HTTP || *self == RESULT_PROPERTY_IRI_HTTPS
	}
}
pub struct ResultPropertyIriOrLabel;
impl PartialEq<&str> for ResultPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResultPropertyIri || *other == RESULT_PROPERTY_LABEL
	}
}
impl PartialEq<ResultPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ResultPropertyIriOrLabel) -> bool {
		*self == ResultPropertyIri || *self == RESULT_PROPERTY_LABEL
	}
}
