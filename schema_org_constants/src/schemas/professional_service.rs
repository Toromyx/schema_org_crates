/// <https://schema.org/ProfessionalService>
pub const PROFESSIONAL_SERVICE_IRI_HTTP: &str = "http://schema.org/ProfessionalService";
/// <https://schema.org/ProfessionalService>
pub const PROFESSIONAL_SERVICE_IRI_HTTPS: &str = "https://schema.org/ProfessionalService";
/// <https://schema.org/ProfessionalService>
pub const PROFESSIONAL_SERVICE_LABEL: &str = "ProfessionalService";
pub struct ProfessionalServiceIri;
impl PartialEq<&str> for ProfessionalServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROFESSIONAL_SERVICE_IRI_HTTP || *other == PROFESSIONAL_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<ProfessionalServiceIri> for &str {
	fn eq(&self, other: &ProfessionalServiceIri) -> bool {
		*self == PROFESSIONAL_SERVICE_IRI_HTTP || *self == PROFESSIONAL_SERVICE_IRI_HTTPS
	}
}
pub struct ProfessionalServiceIriOrLabel;
impl PartialEq<&str> for ProfessionalServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProfessionalServiceIri || *other == PROFESSIONAL_SERVICE_LABEL
	}
}
impl PartialEq<ProfessionalServiceIriOrLabel> for &str {
	fn eq(&self, other: &ProfessionalServiceIriOrLabel) -> bool {
		*self == ProfessionalServiceIri || *self == PROFESSIONAL_SERVICE_LABEL
	}
}
