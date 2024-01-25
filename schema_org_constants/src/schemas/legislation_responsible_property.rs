/// <https://schema.org/legislationResponsible>
pub const LEGISLATION_RESPONSIBLE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/legislationResponsible";
/// <https://schema.org/legislationResponsible>
pub const LEGISLATION_RESPONSIBLE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/legislationResponsible";
/// <https://schema.org/legislationResponsible>
pub const LEGISLATION_RESPONSIBLE_PROPERTY_LABEL: &str = "legislationResponsible";
pub struct LegislationResponsiblePropertyIri;
impl PartialEq<&str> for LegislationResponsiblePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_RESPONSIBLE_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_RESPONSIBLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationResponsiblePropertyIri> for &str {
	fn eq(&self, other: &LegislationResponsiblePropertyIri) -> bool {
		*self == LEGISLATION_RESPONSIBLE_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_RESPONSIBLE_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationResponsiblePropertyIriOrLabel;
impl PartialEq<&str> for LegislationResponsiblePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationResponsiblePropertyIri
			|| *other == LEGISLATION_RESPONSIBLE_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationResponsiblePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationResponsiblePropertyIriOrLabel) -> bool {
		*self == LegislationResponsiblePropertyIri
			|| *self == LEGISLATION_RESPONSIBLE_PROPERTY_LABEL
	}
}
