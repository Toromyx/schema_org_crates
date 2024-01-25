/// <https://schema.org/legislationTransposes>
pub const LEGISLATION_TRANSPOSES_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/legislationTransposes";
/// <https://schema.org/legislationTransposes>
pub const LEGISLATION_TRANSPOSES_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/legislationTransposes";
/// <https://schema.org/legislationTransposes>
pub const LEGISLATION_TRANSPOSES_PROPERTY_LABEL: &str = "legislationTransposes";
pub struct LegislationTransposesPropertyIri;
impl PartialEq<&str> for LegislationTransposesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_TRANSPOSES_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_TRANSPOSES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationTransposesPropertyIri> for &str {
	fn eq(&self, other: &LegislationTransposesPropertyIri) -> bool {
		*self == LEGISLATION_TRANSPOSES_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_TRANSPOSES_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationTransposesPropertyIriOrLabel;
impl PartialEq<&str> for LegislationTransposesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationTransposesPropertyIri
			|| *other == LEGISLATION_TRANSPOSES_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationTransposesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationTransposesPropertyIriOrLabel) -> bool {
		*self == LegislationTransposesPropertyIri || *self == LEGISLATION_TRANSPOSES_PROPERTY_LABEL
	}
}
