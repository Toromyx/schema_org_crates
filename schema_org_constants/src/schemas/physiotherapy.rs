/// <https://schema.org/Physiotherapy>
pub const PHYSIOTHERAPY_IRI_HTTP: &str = "http://schema.org/Physiotherapy";
/// <https://schema.org/Physiotherapy>
pub const PHYSIOTHERAPY_IRI_HTTPS: &str = "https://schema.org/Physiotherapy";
/// <https://schema.org/Physiotherapy>
pub const PHYSIOTHERAPY_LABEL: &str = "Physiotherapy";
pub struct PhysiotherapyIri;
impl PartialEq<&str> for PhysiotherapyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHYSIOTHERAPY_IRI_HTTP || *other == PHYSIOTHERAPY_IRI_HTTPS
	}
}
impl PartialEq<PhysiotherapyIri> for &str {
	fn eq(&self, other: &PhysiotherapyIri) -> bool {
		*self == PHYSIOTHERAPY_IRI_HTTP || *self == PHYSIOTHERAPY_IRI_HTTPS
	}
}
pub struct PhysiotherapyIriOrLabel;
impl PartialEq<&str> for PhysiotherapyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhysiotherapyIri || *other == PHYSIOTHERAPY_LABEL
	}
}
impl PartialEq<PhysiotherapyIriOrLabel> for &str {
	fn eq(&self, other: &PhysiotherapyIriOrLabel) -> bool {
		*self == PhysiotherapyIri || *self == PHYSIOTHERAPY_LABEL
	}
}
