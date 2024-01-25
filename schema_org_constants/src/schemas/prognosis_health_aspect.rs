/// <https://schema.org/PrognosisHealthAspect>
pub const PROGNOSIS_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/PrognosisHealthAspect";
/// <https://schema.org/PrognosisHealthAspect>
pub const PROGNOSIS_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/PrognosisHealthAspect";
/// <https://schema.org/PrognosisHealthAspect>
pub const PROGNOSIS_HEALTH_ASPECT_LABEL: &str = "PrognosisHealthAspect";
pub struct PrognosisHealthAspectIri;
impl PartialEq<&str> for PrognosisHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROGNOSIS_HEALTH_ASPECT_IRI_HTTP || *other == PROGNOSIS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<PrognosisHealthAspectIri> for &str {
	fn eq(&self, other: &PrognosisHealthAspectIri) -> bool {
		*self == PROGNOSIS_HEALTH_ASPECT_IRI_HTTP || *self == PROGNOSIS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct PrognosisHealthAspectIriOrLabel;
impl PartialEq<&str> for PrognosisHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrognosisHealthAspectIri || *other == PROGNOSIS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<PrognosisHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &PrognosisHealthAspectIriOrLabel) -> bool {
		*self == PrognosisHealthAspectIri || *self == PROGNOSIS_HEALTH_ASPECT_LABEL
	}
}
