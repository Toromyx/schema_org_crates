/// <https://schema.org/BankOrCreditUnion>
pub const BANK_OR_CREDIT_UNION_IRI_HTTP: &str = "http://schema.org/BankOrCreditUnion";
/// <https://schema.org/BankOrCreditUnion>
pub const BANK_OR_CREDIT_UNION_IRI_HTTPS: &str = "https://schema.org/BankOrCreditUnion";
/// <https://schema.org/BankOrCreditUnion>
pub const BANK_OR_CREDIT_UNION_LABEL: &str = "BankOrCreditUnion";
pub struct BankOrCreditUnionIri;
impl PartialEq<&str> for BankOrCreditUnionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BANK_OR_CREDIT_UNION_IRI_HTTP || *other == BANK_OR_CREDIT_UNION_IRI_HTTPS
	}
}
impl PartialEq<BankOrCreditUnionIri> for &str {
	fn eq(&self, other: &BankOrCreditUnionIri) -> bool {
		*self == BANK_OR_CREDIT_UNION_IRI_HTTP || *self == BANK_OR_CREDIT_UNION_IRI_HTTPS
	}
}
pub struct BankOrCreditUnionIriOrLabel;
impl PartialEq<&str> for BankOrCreditUnionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BankOrCreditUnionIri || *other == BANK_OR_CREDIT_UNION_LABEL
	}
}
impl PartialEq<BankOrCreditUnionIriOrLabel> for &str {
	fn eq(&self, other: &BankOrCreditUnionIriOrLabel) -> bool {
		*self == BankOrCreditUnionIri || *self == BANK_OR_CREDIT_UNION_LABEL
	}
}
