/// <https://schema.org/possibleComplication>
pub const POSSIBLE_COMPLICATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/possibleComplication";
/// <https://schema.org/possibleComplication>
pub const POSSIBLE_COMPLICATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/possibleComplication";
/// <https://schema.org/possibleComplication>
pub const POSSIBLE_COMPLICATION_PROPERTY_LABEL: &str = "possibleComplication";
pub struct PossibleComplicationPropertyIri;
impl PartialEq<&str> for PossibleComplicationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSSIBLE_COMPLICATION_PROPERTY_IRI_HTTP
			|| *other == POSSIBLE_COMPLICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PossibleComplicationPropertyIri> for &str {
	fn eq(&self, other: &PossibleComplicationPropertyIri) -> bool {
		*self == POSSIBLE_COMPLICATION_PROPERTY_IRI_HTTP
			|| *self == POSSIBLE_COMPLICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct PossibleComplicationPropertyIriOrLabel;
impl PartialEq<&str> for PossibleComplicationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PossibleComplicationPropertyIri || *other == POSSIBLE_COMPLICATION_PROPERTY_LABEL
	}
}
impl PartialEq<PossibleComplicationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PossibleComplicationPropertyIriOrLabel) -> bool {
		*self == PossibleComplicationPropertyIri || *self == POSSIBLE_COMPLICATION_PROPERTY_LABEL
	}
}
