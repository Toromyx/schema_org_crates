/// <https://schema.org/legislationPassedBy>
pub const LEGISLATION_PASSED_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/legislationPassedBy";
/// <https://schema.org/legislationPassedBy>
pub const LEGISLATION_PASSED_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/legislationPassedBy";
/// <https://schema.org/legislationPassedBy>
pub const LEGISLATION_PASSED_BY_PROPERTY_LABEL: &str = "legislationPassedBy";
pub struct LegislationPassedByPropertyIri;
impl PartialEq<&str> for LegislationPassedByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_PASSED_BY_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_PASSED_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationPassedByPropertyIri> for &str {
	fn eq(&self, other: &LegislationPassedByPropertyIri) -> bool {
		*self == LEGISLATION_PASSED_BY_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_PASSED_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationPassedByPropertyIriOrLabel;
impl PartialEq<&str> for LegislationPassedByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationPassedByPropertyIri || *other == LEGISLATION_PASSED_BY_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationPassedByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationPassedByPropertyIriOrLabel) -> bool {
		*self == LegislationPassedByPropertyIri || *self == LEGISLATION_PASSED_BY_PROPERTY_LABEL
	}
}
