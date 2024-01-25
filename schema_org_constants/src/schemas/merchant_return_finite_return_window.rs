/// <https://schema.org/MerchantReturnFiniteReturnWindow>
pub const MERCHANT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTP: &str =
	"http://schema.org/MerchantReturnFiniteReturnWindow";
/// <https://schema.org/MerchantReturnFiniteReturnWindow>
pub const MERCHANT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTPS: &str =
	"https://schema.org/MerchantReturnFiniteReturnWindow";
/// <https://schema.org/MerchantReturnFiniteReturnWindow>
pub const MERCHANT_RETURN_FINITE_RETURN_WINDOW_LABEL: &str = "MerchantReturnFiniteReturnWindow";
pub struct MerchantReturnFiniteReturnWindowIri;
impl PartialEq<&str> for MerchantReturnFiniteReturnWindowIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MERCHANT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTP
			|| *other == MERCHANT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTPS
	}
}
impl PartialEq<MerchantReturnFiniteReturnWindowIri> for &str {
	fn eq(&self, other: &MerchantReturnFiniteReturnWindowIri) -> bool {
		*self == MERCHANT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTP
			|| *self == MERCHANT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTPS
	}
}
pub struct MerchantReturnFiniteReturnWindowIriOrLabel;
impl PartialEq<&str> for MerchantReturnFiniteReturnWindowIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MerchantReturnFiniteReturnWindowIri
			|| *other == MERCHANT_RETURN_FINITE_RETURN_WINDOW_LABEL
	}
}
impl PartialEq<MerchantReturnFiniteReturnWindowIriOrLabel> for &str {
	fn eq(&self, other: &MerchantReturnFiniteReturnWindowIriOrLabel) -> bool {
		*self == MerchantReturnFiniteReturnWindowIri
			|| *self == MERCHANT_RETURN_FINITE_RETURN_WINDOW_LABEL
	}
}
