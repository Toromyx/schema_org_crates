/// <https://schema.org/legislationConsolidates>
pub const LEGISLATION_CONSOLIDATES_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/legislationConsolidates";
/// <https://schema.org/legislationConsolidates>
pub const LEGISLATION_CONSOLIDATES_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/legislationConsolidates";
/// <https://schema.org/legislationConsolidates>
pub const LEGISLATION_CONSOLIDATES_PROPERTY_LABEL: &str = "legislationConsolidates";
pub struct LegislationConsolidatesPropertyIri;
impl PartialEq<&str> for LegislationConsolidatesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_CONSOLIDATES_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_CONSOLIDATES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationConsolidatesPropertyIri> for &str {
	fn eq(&self, other: &LegislationConsolidatesPropertyIri) -> bool {
		*self == LEGISLATION_CONSOLIDATES_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_CONSOLIDATES_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationConsolidatesPropertyIriOrLabel;
impl PartialEq<&str> for LegislationConsolidatesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationConsolidatesPropertyIri
			|| *other == LEGISLATION_CONSOLIDATES_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationConsolidatesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationConsolidatesPropertyIriOrLabel) -> bool {
		*self == LegislationConsolidatesPropertyIri
			|| *self == LEGISLATION_CONSOLIDATES_PROPERTY_LABEL
	}
}
