/// <https://schema.org/DatedMoneySpecification>
#[deprecated = "This schema is superseded by <https://schema.org/MonetaryAmount>."]
pub const DATED_MONEY_SPECIFICATION_IRI_HTTP: &str = "http://schema.org/DatedMoneySpecification";
/// <https://schema.org/DatedMoneySpecification>
#[deprecated = "This schema is superseded by <https://schema.org/MonetaryAmount>."]
pub const DATED_MONEY_SPECIFICATION_IRI_HTTPS: &str = "https://schema.org/DatedMoneySpecification";
/// <https://schema.org/DatedMoneySpecification>
#[deprecated = "This schema is superseded by <https://schema.org/MonetaryAmount>."]
pub const DATED_MONEY_SPECIFICATION_LABEL: &str = "DatedMoneySpecification";
pub struct DatedMoneySpecificationIri;
impl PartialEq<&str> for DatedMoneySpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATED_MONEY_SPECIFICATION_IRI_HTTP
			|| *other == DATED_MONEY_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<DatedMoneySpecificationIri> for &str {
	fn eq(&self, other: &DatedMoneySpecificationIri) -> bool {
		*self == DATED_MONEY_SPECIFICATION_IRI_HTTP || *self == DATED_MONEY_SPECIFICATION_IRI_HTTPS
	}
}
pub struct DatedMoneySpecificationIriOrLabel;
impl PartialEq<&str> for DatedMoneySpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DatedMoneySpecificationIri || *other == DATED_MONEY_SPECIFICATION_LABEL
	}
}
impl PartialEq<DatedMoneySpecificationIriOrLabel> for &str {
	fn eq(&self, other: &DatedMoneySpecificationIriOrLabel) -> bool {
		*self == DatedMoneySpecificationIri || *self == DATED_MONEY_SPECIFICATION_LABEL
	}
}
