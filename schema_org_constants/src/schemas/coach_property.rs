/// <https://schema.org/coach>
pub const COACH_PROPERTY_IRI_HTTP: &str = "http://schema.org/coach";
/// <https://schema.org/coach>
pub const COACH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/coach";
/// <https://schema.org/coach>
pub const COACH_PROPERTY_LABEL: &str = "coach";
pub struct CoachPropertyIri;
impl PartialEq<&str> for CoachPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COACH_PROPERTY_IRI_HTTP || *other == COACH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CoachPropertyIri> for &str {
	fn eq(&self, other: &CoachPropertyIri) -> bool {
		*self == COACH_PROPERTY_IRI_HTTP || *self == COACH_PROPERTY_IRI_HTTPS
	}
}
pub struct CoachPropertyIriOrLabel;
impl PartialEq<&str> for CoachPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CoachPropertyIri || *other == COACH_PROPERTY_LABEL
	}
}
impl PartialEq<CoachPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CoachPropertyIriOrLabel) -> bool {
		*self == CoachPropertyIri || *self == COACH_PROPERTY_LABEL
	}
}
