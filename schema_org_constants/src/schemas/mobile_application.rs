/// <https://schema.org/MobileApplication>
pub const MOBILE_APPLICATION_IRI_HTTP: &str = "http://schema.org/MobileApplication";
/// <https://schema.org/MobileApplication>
pub const MOBILE_APPLICATION_IRI_HTTPS: &str = "https://schema.org/MobileApplication";
/// <https://schema.org/MobileApplication>
pub const MOBILE_APPLICATION_LABEL: &str = "MobileApplication";
pub struct MobileApplicationIri;
impl PartialEq<&str> for MobileApplicationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOBILE_APPLICATION_IRI_HTTP || *other == MOBILE_APPLICATION_IRI_HTTPS
	}
}
impl PartialEq<MobileApplicationIri> for &str {
	fn eq(&self, other: &MobileApplicationIri) -> bool {
		*self == MOBILE_APPLICATION_IRI_HTTP || *self == MOBILE_APPLICATION_IRI_HTTPS
	}
}
pub struct MobileApplicationIriOrLabel;
impl PartialEq<&str> for MobileApplicationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MobileApplicationIri || *other == MOBILE_APPLICATION_LABEL
	}
}
impl PartialEq<MobileApplicationIriOrLabel> for &str {
	fn eq(&self, other: &MobileApplicationIriOrLabel) -> bool {
		*self == MobileApplicationIri || *self == MOBILE_APPLICATION_LABEL
	}
}
