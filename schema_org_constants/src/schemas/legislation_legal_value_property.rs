/// <https://schema.org/legislationLegalValue>
pub const LEGISLATION_LEGAL_VALUE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/legislationLegalValue";
/// <https://schema.org/legislationLegalValue>
pub const LEGISLATION_LEGAL_VALUE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/legislationLegalValue";
/// <https://schema.org/legislationLegalValue>
pub const LEGISLATION_LEGAL_VALUE_PROPERTY_LABEL: &str = "legislationLegalValue";
pub struct LegislationLegalValuePropertyIri;
impl PartialEq<&str> for LegislationLegalValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_LEGAL_VALUE_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_LEGAL_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationLegalValuePropertyIri> for &str {
	fn eq(&self, other: &LegislationLegalValuePropertyIri) -> bool {
		*self == LEGISLATION_LEGAL_VALUE_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_LEGAL_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationLegalValuePropertyIriOrLabel;
impl PartialEq<&str> for LegislationLegalValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationLegalValuePropertyIri
			|| *other == LEGISLATION_LEGAL_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationLegalValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationLegalValuePropertyIriOrLabel) -> bool {
		*self == LegislationLegalValuePropertyIri || *self == LEGISLATION_LEGAL_VALUE_PROPERTY_LABEL
	}
}
