/// <https://schema.org/BusinessSupport>
pub const BUSINESS_SUPPORT_IRI_HTTP: &str = "http://schema.org/BusinessSupport";
/// <https://schema.org/BusinessSupport>
pub const BUSINESS_SUPPORT_IRI_HTTPS: &str = "https://schema.org/BusinessSupport";
/// <https://schema.org/BusinessSupport>
pub const BUSINESS_SUPPORT_LABEL: &str = "BusinessSupport";
pub struct BusinessSupportIri;
impl PartialEq<&str> for BusinessSupportIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUSINESS_SUPPORT_IRI_HTTP || *other == BUSINESS_SUPPORT_IRI_HTTPS
	}
}
impl PartialEq<BusinessSupportIri> for &str {
	fn eq(&self, other: &BusinessSupportIri) -> bool {
		*self == BUSINESS_SUPPORT_IRI_HTTP || *self == BUSINESS_SUPPORT_IRI_HTTPS
	}
}
pub struct BusinessSupportIriOrLabel;
impl PartialEq<&str> for BusinessSupportIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusinessSupportIri || *other == BUSINESS_SUPPORT_LABEL
	}
}
impl PartialEq<BusinessSupportIriOrLabel> for &str {
	fn eq(&self, other: &BusinessSupportIriOrLabel) -> bool {
		*self == BusinessSupportIri || *self == BUSINESS_SUPPORT_LABEL
	}
}
