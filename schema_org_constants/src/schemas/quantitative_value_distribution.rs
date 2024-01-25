/// <https://schema.org/QuantitativeValueDistribution>
pub const QUANTITATIVE_VALUE_DISTRIBUTION_IRI_HTTP: &str =
	"http://schema.org/QuantitativeValueDistribution";
/// <https://schema.org/QuantitativeValueDistribution>
pub const QUANTITATIVE_VALUE_DISTRIBUTION_IRI_HTTPS: &str =
	"https://schema.org/QuantitativeValueDistribution";
/// <https://schema.org/QuantitativeValueDistribution>
pub const QUANTITATIVE_VALUE_DISTRIBUTION_LABEL: &str = "QuantitativeValueDistribution";
pub struct QuantitativeValueDistributionIri;
impl PartialEq<&str> for QuantitativeValueDistributionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUANTITATIVE_VALUE_DISTRIBUTION_IRI_HTTP
			|| *other == QUANTITATIVE_VALUE_DISTRIBUTION_IRI_HTTPS
	}
}
impl PartialEq<QuantitativeValueDistributionIri> for &str {
	fn eq(&self, other: &QuantitativeValueDistributionIri) -> bool {
		*self == QUANTITATIVE_VALUE_DISTRIBUTION_IRI_HTTP
			|| *self == QUANTITATIVE_VALUE_DISTRIBUTION_IRI_HTTPS
	}
}
pub struct QuantitativeValueDistributionIriOrLabel;
impl PartialEq<&str> for QuantitativeValueDistributionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QuantitativeValueDistributionIri
			|| *other == QUANTITATIVE_VALUE_DISTRIBUTION_LABEL
	}
}
impl PartialEq<QuantitativeValueDistributionIriOrLabel> for &str {
	fn eq(&self, other: &QuantitativeValueDistributionIriOrLabel) -> bool {
		*self == QuantitativeValueDistributionIri || *self == QUANTITATIVE_VALUE_DISTRIBUTION_LABEL
	}
}
