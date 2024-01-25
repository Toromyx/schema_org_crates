/// <https://schema.org/potentialAction>
pub const POTENTIAL_ACTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/potentialAction";
/// <https://schema.org/potentialAction>
pub const POTENTIAL_ACTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/potentialAction";
/// <https://schema.org/potentialAction>
pub const POTENTIAL_ACTION_PROPERTY_LABEL: &str = "potentialAction";
pub struct PotentialActionPropertyIri;
impl PartialEq<&str> for PotentialActionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POTENTIAL_ACTION_PROPERTY_IRI_HTTP
			|| *other == POTENTIAL_ACTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PotentialActionPropertyIri> for &str {
	fn eq(&self, other: &PotentialActionPropertyIri) -> bool {
		*self == POTENTIAL_ACTION_PROPERTY_IRI_HTTP || *self == POTENTIAL_ACTION_PROPERTY_IRI_HTTPS
	}
}
pub struct PotentialActionPropertyIriOrLabel;
impl PartialEq<&str> for PotentialActionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PotentialActionPropertyIri || *other == POTENTIAL_ACTION_PROPERTY_LABEL
	}
}
impl PartialEq<PotentialActionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PotentialActionPropertyIriOrLabel) -> bool {
		*self == PotentialActionPropertyIri || *self == POTENTIAL_ACTION_PROPERTY_LABEL
	}
}
