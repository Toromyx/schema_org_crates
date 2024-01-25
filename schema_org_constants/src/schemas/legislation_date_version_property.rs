/// <https://schema.org/legislationDateVersion>
pub const LEGISLATION_DATE_VERSION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/legislationDateVersion";
/// <https://schema.org/legislationDateVersion>
pub const LEGISLATION_DATE_VERSION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/legislationDateVersion";
/// <https://schema.org/legislationDateVersion>
pub const LEGISLATION_DATE_VERSION_PROPERTY_LABEL: &str = "legislationDateVersion";
pub struct LegislationDateVersionPropertyIri;
impl PartialEq<&str> for LegislationDateVersionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_DATE_VERSION_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_DATE_VERSION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationDateVersionPropertyIri> for &str {
	fn eq(&self, other: &LegislationDateVersionPropertyIri) -> bool {
		*self == LEGISLATION_DATE_VERSION_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_DATE_VERSION_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationDateVersionPropertyIriOrLabel;
impl PartialEq<&str> for LegislationDateVersionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationDateVersionPropertyIri
			|| *other == LEGISLATION_DATE_VERSION_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationDateVersionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationDateVersionPropertyIriOrLabel) -> bool {
		*self == LegislationDateVersionPropertyIri
			|| *self == LEGISLATION_DATE_VERSION_PROPERTY_LABEL
	}
}
