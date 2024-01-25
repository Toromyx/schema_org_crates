/// <https://schema.org/RepaymentSpecification>
pub const REPAYMENT_SPECIFICATION_IRI_HTTP: &str = "http://schema.org/RepaymentSpecification";
/// <https://schema.org/RepaymentSpecification>
pub const REPAYMENT_SPECIFICATION_IRI_HTTPS: &str = "https://schema.org/RepaymentSpecification";
/// <https://schema.org/RepaymentSpecification>
pub const REPAYMENT_SPECIFICATION_LABEL: &str = "RepaymentSpecification";
pub struct RepaymentSpecificationIri;
impl PartialEq<&str> for RepaymentSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPAYMENT_SPECIFICATION_IRI_HTTP || *other == REPAYMENT_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<RepaymentSpecificationIri> for &str {
	fn eq(&self, other: &RepaymentSpecificationIri) -> bool {
		*self == REPAYMENT_SPECIFICATION_IRI_HTTP || *self == REPAYMENT_SPECIFICATION_IRI_HTTPS
	}
}
pub struct RepaymentSpecificationIriOrLabel;
impl PartialEq<&str> for RepaymentSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RepaymentSpecificationIri || *other == REPAYMENT_SPECIFICATION_LABEL
	}
}
impl PartialEq<RepaymentSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &RepaymentSpecificationIriOrLabel) -> bool {
		*self == RepaymentSpecificationIri || *self == REPAYMENT_SPECIFICATION_LABEL
	}
}
