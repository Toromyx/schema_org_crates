/// <https://schema.org/ReturnAtKiosk>
pub const RETURN_AT_KIOSK_IRI_HTTP: &str = "http://schema.org/ReturnAtKiosk";
/// <https://schema.org/ReturnAtKiosk>
pub const RETURN_AT_KIOSK_IRI_HTTPS: &str = "https://schema.org/ReturnAtKiosk";
/// <https://schema.org/ReturnAtKiosk>
pub const RETURN_AT_KIOSK_LABEL: &str = "ReturnAtKiosk";
pub struct ReturnAtKioskIri;
impl PartialEq<&str> for ReturnAtKioskIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_AT_KIOSK_IRI_HTTP || *other == RETURN_AT_KIOSK_IRI_HTTPS
	}
}
impl PartialEq<ReturnAtKioskIri> for &str {
	fn eq(&self, other: &ReturnAtKioskIri) -> bool {
		*self == RETURN_AT_KIOSK_IRI_HTTP || *self == RETURN_AT_KIOSK_IRI_HTTPS
	}
}
pub struct ReturnAtKioskIriOrLabel;
impl PartialEq<&str> for ReturnAtKioskIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnAtKioskIri || *other == RETURN_AT_KIOSK_LABEL
	}
}
impl PartialEq<ReturnAtKioskIriOrLabel> for &str {
	fn eq(&self, other: &ReturnAtKioskIriOrLabel) -> bool {
		*self == ReturnAtKioskIri || *self == RETURN_AT_KIOSK_LABEL
	}
}
