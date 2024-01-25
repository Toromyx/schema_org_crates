/// <https://schema.org/legislationJurisdiction>
pub const LEGISLATION_JURISDICTION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/legislationJurisdiction";
/// <https://schema.org/legislationJurisdiction>
pub const LEGISLATION_JURISDICTION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/legislationJurisdiction";
/// <https://schema.org/legislationJurisdiction>
pub const LEGISLATION_JURISDICTION_PROPERTY_LABEL: &str = "legislationJurisdiction";
pub struct LegislationJurisdictionPropertyIri;
impl PartialEq<&str> for LegislationJurisdictionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_JURISDICTION_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_JURISDICTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationJurisdictionPropertyIri> for &str {
	fn eq(&self, other: &LegislationJurisdictionPropertyIri) -> bool {
		*self == LEGISLATION_JURISDICTION_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_JURISDICTION_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationJurisdictionPropertyIriOrLabel;
impl PartialEq<&str> for LegislationJurisdictionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationJurisdictionPropertyIri
			|| *other == LEGISLATION_JURISDICTION_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationJurisdictionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationJurisdictionPropertyIriOrLabel) -> bool {
		*self == LegislationJurisdictionPropertyIri
			|| *self == LEGISLATION_JURISDICTION_PROPERTY_LABEL
	}
}
