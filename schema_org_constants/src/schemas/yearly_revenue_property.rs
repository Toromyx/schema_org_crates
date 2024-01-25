/// <https://schema.org/yearlyRevenue>
pub const YEARLY_REVENUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/yearlyRevenue";
/// <https://schema.org/yearlyRevenue>
pub const YEARLY_REVENUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/yearlyRevenue";
/// <https://schema.org/yearlyRevenue>
pub const YEARLY_REVENUE_PROPERTY_LABEL: &str = "yearlyRevenue";
pub struct YearlyRevenuePropertyIri;
impl PartialEq<&str> for YearlyRevenuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == YEARLY_REVENUE_PROPERTY_IRI_HTTP || *other == YEARLY_REVENUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<YearlyRevenuePropertyIri> for &str {
	fn eq(&self, other: &YearlyRevenuePropertyIri) -> bool {
		*self == YEARLY_REVENUE_PROPERTY_IRI_HTTP || *self == YEARLY_REVENUE_PROPERTY_IRI_HTTPS
	}
}
pub struct YearlyRevenuePropertyIriOrLabel;
impl PartialEq<&str> for YearlyRevenuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == YearlyRevenuePropertyIri || *other == YEARLY_REVENUE_PROPERTY_LABEL
	}
}
impl PartialEq<YearlyRevenuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &YearlyRevenuePropertyIriOrLabel) -> bool {
		*self == YearlyRevenuePropertyIri || *self == YEARLY_REVENUE_PROPERTY_LABEL
	}
}
