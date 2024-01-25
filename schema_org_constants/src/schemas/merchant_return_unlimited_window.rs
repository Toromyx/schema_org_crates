/// <https://schema.org/MerchantReturnUnlimitedWindow>
pub const MERCHANT_RETURN_UNLIMITED_WINDOW_IRI_HTTP: &str =
	"http://schema.org/MerchantReturnUnlimitedWindow";
/// <https://schema.org/MerchantReturnUnlimitedWindow>
pub const MERCHANT_RETURN_UNLIMITED_WINDOW_IRI_HTTPS: &str =
	"https://schema.org/MerchantReturnUnlimitedWindow";
/// <https://schema.org/MerchantReturnUnlimitedWindow>
pub const MERCHANT_RETURN_UNLIMITED_WINDOW_LABEL: &str = "MerchantReturnUnlimitedWindow";
pub struct MerchantReturnUnlimitedWindowIri;
impl PartialEq<&str> for MerchantReturnUnlimitedWindowIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MERCHANT_RETURN_UNLIMITED_WINDOW_IRI_HTTP
			|| *other == MERCHANT_RETURN_UNLIMITED_WINDOW_IRI_HTTPS
	}
}
impl PartialEq<MerchantReturnUnlimitedWindowIri> for &str {
	fn eq(&self, other: &MerchantReturnUnlimitedWindowIri) -> bool {
		*self == MERCHANT_RETURN_UNLIMITED_WINDOW_IRI_HTTP
			|| *self == MERCHANT_RETURN_UNLIMITED_WINDOW_IRI_HTTPS
	}
}
pub struct MerchantReturnUnlimitedWindowIriOrLabel;
impl PartialEq<&str> for MerchantReturnUnlimitedWindowIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MerchantReturnUnlimitedWindowIri
			|| *other == MERCHANT_RETURN_UNLIMITED_WINDOW_LABEL
	}
}
impl PartialEq<MerchantReturnUnlimitedWindowIriOrLabel> for &str {
	fn eq(&self, other: &MerchantReturnUnlimitedWindowIriOrLabel) -> bool {
		*self == MerchantReturnUnlimitedWindowIri || *self == MERCHANT_RETURN_UNLIMITED_WINDOW_LABEL
	}
}
