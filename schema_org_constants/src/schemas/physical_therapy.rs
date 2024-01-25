/// <https://schema.org/PhysicalTherapy>
pub const PHYSICAL_THERAPY_IRI_HTTP: &str = "http://schema.org/PhysicalTherapy";
/// <https://schema.org/PhysicalTherapy>
pub const PHYSICAL_THERAPY_IRI_HTTPS: &str = "https://schema.org/PhysicalTherapy";
/// <https://schema.org/PhysicalTherapy>
pub const PHYSICAL_THERAPY_LABEL: &str = "PhysicalTherapy";
pub struct PhysicalTherapyIri;
impl PartialEq<&str> for PhysicalTherapyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHYSICAL_THERAPY_IRI_HTTP || *other == PHYSICAL_THERAPY_IRI_HTTPS
	}
}
impl PartialEq<PhysicalTherapyIri> for &str {
	fn eq(&self, other: &PhysicalTherapyIri) -> bool {
		*self == PHYSICAL_THERAPY_IRI_HTTP || *self == PHYSICAL_THERAPY_IRI_HTTPS
	}
}
pub struct PhysicalTherapyIriOrLabel;
impl PartialEq<&str> for PhysicalTherapyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhysicalTherapyIri || *other == PHYSICAL_THERAPY_LABEL
	}
}
impl PartialEq<PhysicalTherapyIriOrLabel> for &str {
	fn eq(&self, other: &PhysicalTherapyIriOrLabel) -> bool {
		*self == PhysicalTherapyIri || *self == PHYSICAL_THERAPY_LABEL
	}
}
