/// <https://schema.org/PlasticSurgery>
pub const PLASTIC_SURGERY_IRI_HTTP: &str = "http://schema.org/PlasticSurgery";
/// <https://schema.org/PlasticSurgery>
pub const PLASTIC_SURGERY_IRI_HTTPS: &str = "https://schema.org/PlasticSurgery";
/// <https://schema.org/PlasticSurgery>
pub const PLASTIC_SURGERY_LABEL: &str = "PlasticSurgery";
pub struct PlasticSurgeryIri;
impl PartialEq<&str> for PlasticSurgeryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLASTIC_SURGERY_IRI_HTTP || *other == PLASTIC_SURGERY_IRI_HTTPS
	}
}
impl PartialEq<PlasticSurgeryIri> for &str {
	fn eq(&self, other: &PlasticSurgeryIri) -> bool {
		*self == PLASTIC_SURGERY_IRI_HTTP || *self == PLASTIC_SURGERY_IRI_HTTPS
	}
}
pub struct PlasticSurgeryIriOrLabel;
impl PartialEq<&str> for PlasticSurgeryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlasticSurgeryIri || *other == PLASTIC_SURGERY_LABEL
	}
}
impl PartialEq<PlasticSurgeryIriOrLabel> for &str {
	fn eq(&self, other: &PlasticSurgeryIriOrLabel) -> bool {
		*self == PlasticSurgeryIri || *self == PLASTIC_SURGERY_LABEL
	}
}
