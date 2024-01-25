/// <https://schema.org/MobilePhoneStore>
pub const MOBILE_PHONE_STORE_IRI_HTTP: &str = "http://schema.org/MobilePhoneStore";
/// <https://schema.org/MobilePhoneStore>
pub const MOBILE_PHONE_STORE_IRI_HTTPS: &str = "https://schema.org/MobilePhoneStore";
/// <https://schema.org/MobilePhoneStore>
pub const MOBILE_PHONE_STORE_LABEL: &str = "MobilePhoneStore";
pub struct MobilePhoneStoreIri;
impl PartialEq<&str> for MobilePhoneStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOBILE_PHONE_STORE_IRI_HTTP || *other == MOBILE_PHONE_STORE_IRI_HTTPS
	}
}
impl PartialEq<MobilePhoneStoreIri> for &str {
	fn eq(&self, other: &MobilePhoneStoreIri) -> bool {
		*self == MOBILE_PHONE_STORE_IRI_HTTP || *self == MOBILE_PHONE_STORE_IRI_HTTPS
	}
}
pub struct MobilePhoneStoreIriOrLabel;
impl PartialEq<&str> for MobilePhoneStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MobilePhoneStoreIri || *other == MOBILE_PHONE_STORE_LABEL
	}
}
impl PartialEq<MobilePhoneStoreIriOrLabel> for &str {
	fn eq(&self, other: &MobilePhoneStoreIriOrLabel) -> bool {
		*self == MobilePhoneStoreIri || *self == MOBILE_PHONE_STORE_LABEL
	}
}
