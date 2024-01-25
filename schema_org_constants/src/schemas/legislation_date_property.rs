/// <https://schema.org/legislationDate>
pub const LEGISLATION_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/legislationDate";
/// <https://schema.org/legislationDate>
pub const LEGISLATION_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/legislationDate";
/// <https://schema.org/legislationDate>
pub const LEGISLATION_DATE_PROPERTY_LABEL: &str = "legislationDate";
pub struct LegislationDatePropertyIri;
impl PartialEq<&str> for LegislationDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_DATE_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationDatePropertyIri> for &str {
	fn eq(&self, other: &LegislationDatePropertyIri) -> bool {
		*self == LEGISLATION_DATE_PROPERTY_IRI_HTTP || *self == LEGISLATION_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationDatePropertyIriOrLabel;
impl PartialEq<&str> for LegislationDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationDatePropertyIri || *other == LEGISLATION_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationDatePropertyIriOrLabel) -> bool {
		*self == LegislationDatePropertyIri || *self == LEGISLATION_DATE_PROPERTY_LABEL
	}
}
