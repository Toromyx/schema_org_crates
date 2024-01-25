/// <https://schema.org/legislationApplies>
pub const LEGISLATION_APPLIES_PROPERTY_IRI_HTTP: &str = "http://schema.org/legislationApplies";
/// <https://schema.org/legislationApplies>
pub const LEGISLATION_APPLIES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/legislationApplies";
/// <https://schema.org/legislationApplies>
pub const LEGISLATION_APPLIES_PROPERTY_LABEL: &str = "legislationApplies";
pub struct LegislationAppliesPropertyIri;
impl PartialEq<&str> for LegislationAppliesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_APPLIES_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_APPLIES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationAppliesPropertyIri> for &str {
	fn eq(&self, other: &LegislationAppliesPropertyIri) -> bool {
		*self == LEGISLATION_APPLIES_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_APPLIES_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationAppliesPropertyIriOrLabel;
impl PartialEq<&str> for LegislationAppliesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationAppliesPropertyIri || *other == LEGISLATION_APPLIES_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationAppliesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationAppliesPropertyIriOrLabel) -> bool {
		*self == LegislationAppliesPropertyIri || *self == LEGISLATION_APPLIES_PROPERTY_LABEL
	}
}
