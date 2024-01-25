/// <https://schema.org/MayTreatHealthAspect>
pub const MAY_TREAT_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/MayTreatHealthAspect";
/// <https://schema.org/MayTreatHealthAspect>
pub const MAY_TREAT_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/MayTreatHealthAspect";
/// <https://schema.org/MayTreatHealthAspect>
pub const MAY_TREAT_HEALTH_ASPECT_LABEL: &str = "MayTreatHealthAspect";
pub struct MayTreatHealthAspectIri;
impl PartialEq<&str> for MayTreatHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAY_TREAT_HEALTH_ASPECT_IRI_HTTP || *other == MAY_TREAT_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<MayTreatHealthAspectIri> for &str {
	fn eq(&self, other: &MayTreatHealthAspectIri) -> bool {
		*self == MAY_TREAT_HEALTH_ASPECT_IRI_HTTP || *self == MAY_TREAT_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct MayTreatHealthAspectIriOrLabel;
impl PartialEq<&str> for MayTreatHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MayTreatHealthAspectIri || *other == MAY_TREAT_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<MayTreatHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &MayTreatHealthAspectIriOrLabel) -> bool {
		*self == MayTreatHealthAspectIri || *self == MAY_TREAT_HEALTH_ASPECT_LABEL
	}
}
