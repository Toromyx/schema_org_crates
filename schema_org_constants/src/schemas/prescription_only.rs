/// <https://schema.org/PrescriptionOnly>
pub const PRESCRIPTION_ONLY_IRI_HTTP: &str = "http://schema.org/PrescriptionOnly";
/// <https://schema.org/PrescriptionOnly>
pub const PRESCRIPTION_ONLY_IRI_HTTPS: &str = "https://schema.org/PrescriptionOnly";
/// <https://schema.org/PrescriptionOnly>
pub const PRESCRIPTION_ONLY_LABEL: &str = "PrescriptionOnly";
pub struct PrescriptionOnlyIri;
impl PartialEq<&str> for PrescriptionOnlyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRESCRIPTION_ONLY_IRI_HTTP || *other == PRESCRIPTION_ONLY_IRI_HTTPS
	}
}
impl PartialEq<PrescriptionOnlyIri> for &str {
	fn eq(&self, other: &PrescriptionOnlyIri) -> bool {
		*self == PRESCRIPTION_ONLY_IRI_HTTP || *self == PRESCRIPTION_ONLY_IRI_HTTPS
	}
}
pub struct PrescriptionOnlyIriOrLabel;
impl PartialEq<&str> for PrescriptionOnlyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrescriptionOnlyIri || *other == PRESCRIPTION_ONLY_LABEL
	}
}
impl PartialEq<PrescriptionOnlyIriOrLabel> for &str {
	fn eq(&self, other: &PrescriptionOnlyIriOrLabel) -> bool {
		*self == PrescriptionOnlyIri || *self == PRESCRIPTION_ONLY_LABEL
	}
}
