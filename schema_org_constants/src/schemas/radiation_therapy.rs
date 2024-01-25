/// <https://schema.org/RadiationTherapy>
pub const RADIATION_THERAPY_IRI_HTTP: &str = "http://schema.org/RadiationTherapy";
/// <https://schema.org/RadiationTherapy>
pub const RADIATION_THERAPY_IRI_HTTPS: &str = "https://schema.org/RadiationTherapy";
/// <https://schema.org/RadiationTherapy>
pub const RADIATION_THERAPY_LABEL: &str = "RadiationTherapy";
pub struct RadiationTherapyIri;
impl PartialEq<&str> for RadiationTherapyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RADIATION_THERAPY_IRI_HTTP || *other == RADIATION_THERAPY_IRI_HTTPS
	}
}
impl PartialEq<RadiationTherapyIri> for &str {
	fn eq(&self, other: &RadiationTherapyIri) -> bool {
		*self == RADIATION_THERAPY_IRI_HTTP || *self == RADIATION_THERAPY_IRI_HTTPS
	}
}
pub struct RadiationTherapyIriOrLabel;
impl PartialEq<&str> for RadiationTherapyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RadiationTherapyIri || *other == RADIATION_THERAPY_LABEL
	}
}
impl PartialEq<RadiationTherapyIriOrLabel> for &str {
	fn eq(&self, other: &RadiationTherapyIriOrLabel) -> bool {
		*self == RadiationTherapyIri || *self == RADIATION_THERAPY_LABEL
	}
}
