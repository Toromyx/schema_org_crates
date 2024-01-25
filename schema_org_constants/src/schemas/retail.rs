/// <https://schema.org/Retail>
pub const RETAIL_IRI_HTTP: &str = "http://schema.org/Retail";
/// <https://schema.org/Retail>
pub const RETAIL_IRI_HTTPS: &str = "https://schema.org/Retail";
/// <https://schema.org/Retail>
pub const RETAIL_LABEL: &str = "Retail";
pub struct RetailIri;
impl PartialEq<&str> for RetailIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETAIL_IRI_HTTP || *other == RETAIL_IRI_HTTPS
	}
}
impl PartialEq<RetailIri> for &str {
	fn eq(&self, other: &RetailIri) -> bool {
		*self == RETAIL_IRI_HTTP || *self == RETAIL_IRI_HTTPS
	}
}
pub struct RetailIriOrLabel;
impl PartialEq<&str> for RetailIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RetailIri || *other == RETAIL_LABEL
	}
}
impl PartialEq<RetailIriOrLabel> for &str {
	fn eq(&self, other: &RetailIriOrLabel) -> bool {
		*self == RetailIri || *self == RETAIL_LABEL
	}
}
