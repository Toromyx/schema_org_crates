/// <https://schema.org/legislationIdentifier>
pub const LEGISLATION_IDENTIFIER_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/legislationIdentifier";
/// <https://schema.org/legislationIdentifier>
pub const LEGISLATION_IDENTIFIER_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/legislationIdentifier";
/// <https://schema.org/legislationIdentifier>
pub const LEGISLATION_IDENTIFIER_PROPERTY_LABEL: &str = "legislationIdentifier";
pub struct LegislationIdentifierPropertyIri;
impl PartialEq<&str> for LegislationIdentifierPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_IDENTIFIER_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_IDENTIFIER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationIdentifierPropertyIri> for &str {
	fn eq(&self, other: &LegislationIdentifierPropertyIri) -> bool {
		*self == LEGISLATION_IDENTIFIER_PROPERTY_IRI_HTTP
			|| *self == LEGISLATION_IDENTIFIER_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationIdentifierPropertyIriOrLabel;
impl PartialEq<&str> for LegislationIdentifierPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationIdentifierPropertyIri
			|| *other == LEGISLATION_IDENTIFIER_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationIdentifierPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationIdentifierPropertyIriOrLabel) -> bool {
		*self == LegislationIdentifierPropertyIri || *self == LEGISLATION_IDENTIFIER_PROPERTY_LABEL
	}
}
