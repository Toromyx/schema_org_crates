/// <https://schema.org/Downpayment>
pub const DOWNPAYMENT_IRI_HTTP: &str = "http://schema.org/Downpayment";
/// <https://schema.org/Downpayment>
pub const DOWNPAYMENT_IRI_HTTPS: &str = "https://schema.org/Downpayment";
/// <https://schema.org/Downpayment>
pub const DOWNPAYMENT_LABEL: &str = "Downpayment";
pub struct DownpaymentIri;
impl PartialEq<&str> for DownpaymentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOWNPAYMENT_IRI_HTTP || *other == DOWNPAYMENT_IRI_HTTPS
	}
}
impl PartialEq<DownpaymentIri> for &str {
	fn eq(&self, other: &DownpaymentIri) -> bool {
		*self == DOWNPAYMENT_IRI_HTTP || *self == DOWNPAYMENT_IRI_HTTPS
	}
}
pub struct DownpaymentIriOrLabel;
impl PartialEq<&str> for DownpaymentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DownpaymentIri || *other == DOWNPAYMENT_LABEL
	}
}
impl PartialEq<DownpaymentIriOrLabel> for &str {
	fn eq(&self, other: &DownpaymentIriOrLabel) -> bool {
		*self == DownpaymentIri || *self == DOWNPAYMENT_LABEL
	}
}
