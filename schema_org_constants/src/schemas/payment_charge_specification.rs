/// <https://schema.org/PaymentChargeSpecification>
pub const PAYMENT_CHARGE_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/PaymentChargeSpecification";
/// <https://schema.org/PaymentChargeSpecification>
pub const PAYMENT_CHARGE_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/PaymentChargeSpecification";
/// <https://schema.org/PaymentChargeSpecification>
pub const PAYMENT_CHARGE_SPECIFICATION_LABEL: &str = "PaymentChargeSpecification";
pub struct PaymentChargeSpecificationIri;
impl PartialEq<&str> for PaymentChargeSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_CHARGE_SPECIFICATION_IRI_HTTP
			|| *other == PAYMENT_CHARGE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<PaymentChargeSpecificationIri> for &str {
	fn eq(&self, other: &PaymentChargeSpecificationIri) -> bool {
		*self == PAYMENT_CHARGE_SPECIFICATION_IRI_HTTP
			|| *self == PAYMENT_CHARGE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct PaymentChargeSpecificationIriOrLabel;
impl PartialEq<&str> for PaymentChargeSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentChargeSpecificationIri || *other == PAYMENT_CHARGE_SPECIFICATION_LABEL
	}
}
impl PartialEq<PaymentChargeSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &PaymentChargeSpecificationIriOrLabel) -> bool {
		*self == PaymentChargeSpecificationIri || *self == PAYMENT_CHARGE_SPECIFICATION_LABEL
	}
}
