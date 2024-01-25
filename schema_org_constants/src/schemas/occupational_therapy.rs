/// <https://schema.org/OccupationalTherapy>
pub const OCCUPATIONAL_THERAPY_IRI_HTTP: &str = "http://schema.org/OccupationalTherapy";
/// <https://schema.org/OccupationalTherapy>
pub const OCCUPATIONAL_THERAPY_IRI_HTTPS: &str = "https://schema.org/OccupationalTherapy";
/// <https://schema.org/OccupationalTherapy>
pub const OCCUPATIONAL_THERAPY_LABEL: &str = "OccupationalTherapy";
pub struct OccupationalTherapyIri;
impl PartialEq<&str> for OccupationalTherapyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OCCUPATIONAL_THERAPY_IRI_HTTP || *other == OCCUPATIONAL_THERAPY_IRI_HTTPS
	}
}
impl PartialEq<OccupationalTherapyIri> for &str {
	fn eq(&self, other: &OccupationalTherapyIri) -> bool {
		*self == OCCUPATIONAL_THERAPY_IRI_HTTP || *self == OCCUPATIONAL_THERAPY_IRI_HTTPS
	}
}
pub struct OccupationalTherapyIriOrLabel;
impl PartialEq<&str> for OccupationalTherapyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OccupationalTherapyIri || *other == OCCUPATIONAL_THERAPY_LABEL
	}
}
impl PartialEq<OccupationalTherapyIriOrLabel> for &str {
	fn eq(&self, other: &OccupationalTherapyIriOrLabel) -> bool {
		*self == OccupationalTherapyIri || *self == OCCUPATIONAL_THERAPY_LABEL
	}
}
