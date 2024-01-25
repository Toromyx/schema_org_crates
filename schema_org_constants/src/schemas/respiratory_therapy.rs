/// <https://schema.org/RespiratoryTherapy>
pub const RESPIRATORY_THERAPY_IRI_HTTP: &str = "http://schema.org/RespiratoryTherapy";
/// <https://schema.org/RespiratoryTherapy>
pub const RESPIRATORY_THERAPY_IRI_HTTPS: &str = "https://schema.org/RespiratoryTherapy";
/// <https://schema.org/RespiratoryTherapy>
pub const RESPIRATORY_THERAPY_LABEL: &str = "RespiratoryTherapy";
pub struct RespiratoryTherapyIri;
impl PartialEq<&str> for RespiratoryTherapyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESPIRATORY_THERAPY_IRI_HTTP || *other == RESPIRATORY_THERAPY_IRI_HTTPS
	}
}
impl PartialEq<RespiratoryTherapyIri> for &str {
	fn eq(&self, other: &RespiratoryTherapyIri) -> bool {
		*self == RESPIRATORY_THERAPY_IRI_HTTP || *self == RESPIRATORY_THERAPY_IRI_HTTPS
	}
}
pub struct RespiratoryTherapyIriOrLabel;
impl PartialEq<&str> for RespiratoryTherapyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RespiratoryTherapyIri || *other == RESPIRATORY_THERAPY_LABEL
	}
}
impl PartialEq<RespiratoryTherapyIriOrLabel> for &str {
	fn eq(&self, other: &RespiratoryTherapyIriOrLabel) -> bool {
		*self == RespiratoryTherapyIri || *self == RESPIRATORY_THERAPY_LABEL
	}
}
