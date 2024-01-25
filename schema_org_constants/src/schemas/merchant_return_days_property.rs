/// <https://schema.org/merchantReturnDays>
pub const MERCHANT_RETURN_DAYS_PROPERTY_IRI_HTTP: &str = "http://schema.org/merchantReturnDays";
/// <https://schema.org/merchantReturnDays>
pub const MERCHANT_RETURN_DAYS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/merchantReturnDays";
/// <https://schema.org/merchantReturnDays>
pub const MERCHANT_RETURN_DAYS_PROPERTY_LABEL: &str = "merchantReturnDays";
pub struct MerchantReturnDaysPropertyIri;
impl PartialEq<&str> for MerchantReturnDaysPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MERCHANT_RETURN_DAYS_PROPERTY_IRI_HTTP
			|| *other == MERCHANT_RETURN_DAYS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MerchantReturnDaysPropertyIri> for &str {
	fn eq(&self, other: &MerchantReturnDaysPropertyIri) -> bool {
		*self == MERCHANT_RETURN_DAYS_PROPERTY_IRI_HTTP
			|| *self == MERCHANT_RETURN_DAYS_PROPERTY_IRI_HTTPS
	}
}
pub struct MerchantReturnDaysPropertyIriOrLabel;
impl PartialEq<&str> for MerchantReturnDaysPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MerchantReturnDaysPropertyIri || *other == MERCHANT_RETURN_DAYS_PROPERTY_LABEL
	}
}
impl PartialEq<MerchantReturnDaysPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MerchantReturnDaysPropertyIriOrLabel) -> bool {
		*self == MerchantReturnDaysPropertyIri || *self == MERCHANT_RETURN_DAYS_PROPERTY_LABEL
	}
}
