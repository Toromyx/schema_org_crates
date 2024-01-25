/// <https://schema.org/Nursing>
pub const NURSING_IRI_HTTP: &str = "http://schema.org/Nursing";
/// <https://schema.org/Nursing>
pub const NURSING_IRI_HTTPS: &str = "https://schema.org/Nursing";
/// <https://schema.org/Nursing>
pub const NURSING_LABEL: &str = "Nursing";
pub struct NursingIri;
impl PartialEq<&str> for NursingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NURSING_IRI_HTTP || *other == NURSING_IRI_HTTPS
	}
}
impl PartialEq<NursingIri> for &str {
	fn eq(&self, other: &NursingIri) -> bool {
		*self == NURSING_IRI_HTTP || *self == NURSING_IRI_HTTPS
	}
}
pub struct NursingIriOrLabel;
impl PartialEq<&str> for NursingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NursingIri || *other == NURSING_LABEL
	}
}
impl PartialEq<NursingIriOrLabel> for &str {
	fn eq(&self, other: &NursingIriOrLabel) -> bool {
		*self == NursingIri || *self == NURSING_LABEL
	}
}
