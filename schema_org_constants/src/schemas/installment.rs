/// <https://schema.org/Installment>
pub const INSTALLMENT_IRI_HTTP: &str = "http://schema.org/Installment";
/// <https://schema.org/Installment>
pub const INSTALLMENT_IRI_HTTPS: &str = "https://schema.org/Installment";
/// <https://schema.org/Installment>
pub const INSTALLMENT_LABEL: &str = "Installment";
pub struct InstallmentIri;
impl PartialEq<&str> for InstallmentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INSTALLMENT_IRI_HTTP || *other == INSTALLMENT_IRI_HTTPS
	}
}
impl PartialEq<InstallmentIri> for &str {
	fn eq(&self, other: &InstallmentIri) -> bool {
		*self == INSTALLMENT_IRI_HTTP || *self == INSTALLMENT_IRI_HTTPS
	}
}
pub struct InstallmentIriOrLabel;
impl PartialEq<&str> for InstallmentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InstallmentIri || *other == INSTALLMENT_LABEL
	}
}
impl PartialEq<InstallmentIriOrLabel> for &str {
	fn eq(&self, other: &InstallmentIriOrLabel) -> bool {
		*self == InstallmentIri || *self == INSTALLMENT_LABEL
	}
}
