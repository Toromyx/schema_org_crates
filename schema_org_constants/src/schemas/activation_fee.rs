/// <https://schema.org/ActivationFee>
pub const ACTIVATION_FEE_IRI_HTTP: &str = "http://schema.org/ActivationFee";
/// <https://schema.org/ActivationFee>
pub const ACTIVATION_FEE_IRI_HTTPS: &str = "https://schema.org/ActivationFee";
/// <https://schema.org/ActivationFee>
pub const ACTIVATION_FEE_LABEL: &str = "ActivationFee";
pub struct ActivationFeeIri;
impl PartialEq<&str> for ActivationFeeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTIVATION_FEE_IRI_HTTP || *other == ACTIVATION_FEE_IRI_HTTPS
	}
}
impl PartialEq<ActivationFeeIri> for &str {
	fn eq(&self, other: &ActivationFeeIri) -> bool {
		*self == ACTIVATION_FEE_IRI_HTTP || *self == ACTIVATION_FEE_IRI_HTTPS
	}
}
pub struct ActivationFeeIriOrLabel;
impl PartialEq<&str> for ActivationFeeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActivationFeeIri || *other == ACTIVATION_FEE_LABEL
	}
}
impl PartialEq<ActivationFeeIriOrLabel> for &str {
	fn eq(&self, other: &ActivationFeeIriOrLabel) -> bool {
		*self == ActivationFeeIri || *self == ACTIVATION_FEE_LABEL
	}
}
