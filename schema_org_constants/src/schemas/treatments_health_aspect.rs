/// <https://schema.org/TreatmentsHealthAspect>
pub const TREATMENTS_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/TreatmentsHealthAspect";
/// <https://schema.org/TreatmentsHealthAspect>
pub const TREATMENTS_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/TreatmentsHealthAspect";
/// <https://schema.org/TreatmentsHealthAspect>
pub const TREATMENTS_HEALTH_ASPECT_LABEL: &str = "TreatmentsHealthAspect";
pub struct TreatmentsHealthAspectIri;
impl PartialEq<&str> for TreatmentsHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TREATMENTS_HEALTH_ASPECT_IRI_HTTP || *other == TREATMENTS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<TreatmentsHealthAspectIri> for &str {
	fn eq(&self, other: &TreatmentsHealthAspectIri) -> bool {
		*self == TREATMENTS_HEALTH_ASPECT_IRI_HTTP || *self == TREATMENTS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct TreatmentsHealthAspectIriOrLabel;
impl PartialEq<&str> for TreatmentsHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TreatmentsHealthAspectIri || *other == TREATMENTS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<TreatmentsHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &TreatmentsHealthAspectIriOrLabel) -> bool {
		*self == TreatmentsHealthAspectIri || *self == TREATMENTS_HEALTH_ASPECT_LABEL
	}
}
