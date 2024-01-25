/// <https://schema.org/PotentialActionStatus>
pub const POTENTIAL_ACTION_STATUS_IRI_HTTP: &str = "http://schema.org/PotentialActionStatus";
/// <https://schema.org/PotentialActionStatus>
pub const POTENTIAL_ACTION_STATUS_IRI_HTTPS: &str = "https://schema.org/PotentialActionStatus";
/// <https://schema.org/PotentialActionStatus>
pub const POTENTIAL_ACTION_STATUS_LABEL: &str = "PotentialActionStatus";
pub struct PotentialActionStatusIri;
impl PartialEq<&str> for PotentialActionStatusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POTENTIAL_ACTION_STATUS_IRI_HTTP || *other == POTENTIAL_ACTION_STATUS_IRI_HTTPS
	}
}
impl PartialEq<PotentialActionStatusIri> for &str {
	fn eq(&self, other: &PotentialActionStatusIri) -> bool {
		*self == POTENTIAL_ACTION_STATUS_IRI_HTTP || *self == POTENTIAL_ACTION_STATUS_IRI_HTTPS
	}
}
pub struct PotentialActionStatusIriOrLabel;
impl PartialEq<&str> for PotentialActionStatusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PotentialActionStatusIri || *other == POTENTIAL_ACTION_STATUS_LABEL
	}
}
impl PartialEq<PotentialActionStatusIriOrLabel> for &str {
	fn eq(&self, other: &PotentialActionStatusIriOrLabel) -> bool {
		*self == PotentialActionStatusIri || *self == POTENTIAL_ACTION_STATUS_LABEL
	}
}
