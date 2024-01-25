/// <https://schema.org/legislationChanges>
pub const LEGISLATION_CHANGES_PROPERTY_IRI_HTTP: &str = "http://schema.org/legislationChanges";
/// <https://schema.org/legislationChanges>
pub const LEGISLATION_CHANGES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/legislationChanges";
/// <https://schema.org/legislationChanges>
pub const LEGISLATION_CHANGES_PROPERTY_LABEL: &str = "legislationChanges";
pub struct LegislationChangesPropertyIri;
impl PartialEq<&str> for LegislationChangesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_CHANGES_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_CHANGES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationChangesPropertyIri> for &str {
	fn eq(&self, other: &LegislationChangesPropertyIri) -> bool {
		*self == LEGISLATION_CHANGES_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_CHANGES_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationChangesPropertyIriOrLabel;
impl PartialEq<&str> for LegislationChangesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationChangesPropertyIri || *other == LEGISLATION_CHANGES_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationChangesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationChangesPropertyIriOrLabel) -> bool {
		*self == LegislationChangesPropertyIri || *self == LEGISLATION_CHANGES_PROPERTY_LABEL
	}
}
