/// <https://schema.org/legislationLegalForce>
pub const LEGISLATION_LEGAL_FORCE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/legislationLegalForce";
/// <https://schema.org/legislationLegalForce>
pub const LEGISLATION_LEGAL_FORCE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/legislationLegalForce";
/// <https://schema.org/legislationLegalForce>
pub const LEGISLATION_LEGAL_FORCE_PROPERTY_LABEL: &str = "legislationLegalForce";
pub struct LegislationLegalForcePropertyIri;
impl PartialEq<&str> for LegislationLegalForcePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_LEGAL_FORCE_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_LEGAL_FORCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationLegalForcePropertyIri> for &str {
	fn eq(&self, other: &LegislationLegalForcePropertyIri) -> bool {
		*self == LEGISLATION_LEGAL_FORCE_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_LEGAL_FORCE_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationLegalForcePropertyIriOrLabel;
impl PartialEq<&str> for LegislationLegalForcePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationLegalForcePropertyIri
			|| *other == LEGISLATION_LEGAL_FORCE_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationLegalForcePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationLegalForcePropertyIriOrLabel) -> bool {
		*self == LegislationLegalForcePropertyIri || *self == LEGISLATION_LEGAL_FORCE_PROPERTY_LABEL
	}
}
