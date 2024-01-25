/// <https://schema.org/Pulmonary>
pub const PULMONARY_IRI_HTTP: &str = "http://schema.org/Pulmonary";
/// <https://schema.org/Pulmonary>
pub const PULMONARY_IRI_HTTPS: &str = "https://schema.org/Pulmonary";
/// <https://schema.org/Pulmonary>
pub const PULMONARY_LABEL: &str = "Pulmonary";
pub struct PulmonaryIri;
impl PartialEq<&str> for PulmonaryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PULMONARY_IRI_HTTP || *other == PULMONARY_IRI_HTTPS
	}
}
impl PartialEq<PulmonaryIri> for &str {
	fn eq(&self, other: &PulmonaryIri) -> bool {
		*self == PULMONARY_IRI_HTTP || *self == PULMONARY_IRI_HTTPS
	}
}
pub struct PulmonaryIriOrLabel;
impl PartialEq<&str> for PulmonaryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PulmonaryIri || *other == PULMONARY_LABEL
	}
}
impl PartialEq<PulmonaryIriOrLabel> for &str {
	fn eq(&self, other: &PulmonaryIriOrLabel) -> bool {
		*self == PulmonaryIri || *self == PULMONARY_LABEL
	}
}
