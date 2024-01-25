/// <https://schema.org/beneficiaryBank>
pub const BENEFICIARY_BANK_PROPERTY_IRI_HTTP: &str = "http://schema.org/beneficiaryBank";
/// <https://schema.org/beneficiaryBank>
pub const BENEFICIARY_BANK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/beneficiaryBank";
/// <https://schema.org/beneficiaryBank>
pub const BENEFICIARY_BANK_PROPERTY_LABEL: &str = "beneficiaryBank";
pub struct BeneficiaryBankPropertyIri;
impl PartialEq<&str> for BeneficiaryBankPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BENEFICIARY_BANK_PROPERTY_IRI_HTTP
			|| *other == BENEFICIARY_BANK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BeneficiaryBankPropertyIri> for &str {
	fn eq(&self, other: &BeneficiaryBankPropertyIri) -> bool {
		*self == BENEFICIARY_BANK_PROPERTY_IRI_HTTP || *self == BENEFICIARY_BANK_PROPERTY_IRI_HTTPS
	}
}
pub struct BeneficiaryBankPropertyIriOrLabel;
impl PartialEq<&str> for BeneficiaryBankPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BeneficiaryBankPropertyIri || *other == BENEFICIARY_BANK_PROPERTY_LABEL
	}
}
impl PartialEq<BeneficiaryBankPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BeneficiaryBankPropertyIriOrLabel) -> bool {
		*self == BeneficiaryBankPropertyIri || *self == BENEFICIARY_BANK_PROPERTY_LABEL
	}
}
