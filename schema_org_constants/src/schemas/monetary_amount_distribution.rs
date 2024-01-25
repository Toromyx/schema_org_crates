/// <https://schema.org/MonetaryAmountDistribution>
pub const MONETARY_AMOUNT_DISTRIBUTION_IRI_HTTP: &str =
	"http://schema.org/MonetaryAmountDistribution";
/// <https://schema.org/MonetaryAmountDistribution>
pub const MONETARY_AMOUNT_DISTRIBUTION_IRI_HTTPS: &str =
	"https://schema.org/MonetaryAmountDistribution";
/// <https://schema.org/MonetaryAmountDistribution>
pub const MONETARY_AMOUNT_DISTRIBUTION_LABEL: &str = "MonetaryAmountDistribution";
pub struct MonetaryAmountDistributionIri;
impl PartialEq<&str> for MonetaryAmountDistributionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MONETARY_AMOUNT_DISTRIBUTION_IRI_HTTP
			|| *other == MONETARY_AMOUNT_DISTRIBUTION_IRI_HTTPS
	}
}
impl PartialEq<MonetaryAmountDistributionIri> for &str {
	fn eq(&self, other: &MonetaryAmountDistributionIri) -> bool {
		*self == MONETARY_AMOUNT_DISTRIBUTION_IRI_HTTP
			|| *self == MONETARY_AMOUNT_DISTRIBUTION_IRI_HTTPS
	}
}
pub struct MonetaryAmountDistributionIriOrLabel;
impl PartialEq<&str> for MonetaryAmountDistributionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MonetaryAmountDistributionIri || *other == MONETARY_AMOUNT_DISTRIBUTION_LABEL
	}
}
impl PartialEq<MonetaryAmountDistributionIriOrLabel> for &str {
	fn eq(&self, other: &MonetaryAmountDistributionIriOrLabel) -> bool {
		*self == MonetaryAmountDistributionIri || *self == MONETARY_AMOUNT_DISTRIBUTION_LABEL
	}
}
