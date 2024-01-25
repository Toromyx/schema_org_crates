/// <https://schema.org/mobileUrl>
pub const MOBILE_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/mobileUrl";
/// <https://schema.org/mobileUrl>
pub const MOBILE_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mobileUrl";
/// <https://schema.org/mobileUrl>
pub const MOBILE_URL_PROPERTY_LABEL: &str = "mobileUrl";
pub struct MobileUrlPropertyIri;
impl PartialEq<&str> for MobileUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOBILE_URL_PROPERTY_IRI_HTTP || *other == MOBILE_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MobileUrlPropertyIri> for &str {
	fn eq(&self, other: &MobileUrlPropertyIri) -> bool {
		*self == MOBILE_URL_PROPERTY_IRI_HTTP || *self == MOBILE_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct MobileUrlPropertyIriOrLabel;
impl PartialEq<&str> for MobileUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MobileUrlPropertyIri || *other == MOBILE_URL_PROPERTY_LABEL
	}
}
impl PartialEq<MobileUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MobileUrlPropertyIriOrLabel) -> bool {
		*self == MobileUrlPropertyIri || *self == MOBILE_URL_PROPERTY_LABEL
	}
}
