/// <https://schema.org/marginOfError>
pub const MARGIN_OF_ERROR_PROPERTY_IRI_HTTP: &str = "http://schema.org/marginOfError";
/// <https://schema.org/marginOfError>
pub const MARGIN_OF_ERROR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/marginOfError";
/// <https://schema.org/marginOfError>
pub const MARGIN_OF_ERROR_PROPERTY_LABEL: &str = "marginOfError";
pub struct MarginOfErrorPropertyIri;
impl PartialEq<&str> for MarginOfErrorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MARGIN_OF_ERROR_PROPERTY_IRI_HTTP || *other == MARGIN_OF_ERROR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MarginOfErrorPropertyIri> for &str {
	fn eq(&self, other: &MarginOfErrorPropertyIri) -> bool {
		*self == MARGIN_OF_ERROR_PROPERTY_IRI_HTTP || *self == MARGIN_OF_ERROR_PROPERTY_IRI_HTTPS
	}
}
pub struct MarginOfErrorPropertyIriOrLabel;
impl PartialEq<&str> for MarginOfErrorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MarginOfErrorPropertyIri || *other == MARGIN_OF_ERROR_PROPERTY_LABEL
	}
}
impl PartialEq<MarginOfErrorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MarginOfErrorPropertyIriOrLabel) -> bool {
		*self == MarginOfErrorPropertyIri || *self == MARGIN_OF_ERROR_PROPERTY_LABEL
	}
}
