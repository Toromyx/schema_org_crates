/// <https://schema.org/Dentistry>
pub const DENTISTRY_IRI_HTTP: &str = "http://schema.org/Dentistry";
/// <https://schema.org/Dentistry>
pub const DENTISTRY_IRI_HTTPS: &str = "https://schema.org/Dentistry";
/// <https://schema.org/Dentistry>
pub const DENTISTRY_LABEL: &str = "Dentistry";
pub struct DentistryIri;
impl PartialEq<&str> for DentistryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DENTISTRY_IRI_HTTP || *other == DENTISTRY_IRI_HTTPS
	}
}
impl PartialEq<DentistryIri> for &str {
	fn eq(&self, other: &DentistryIri) -> bool {
		*self == DENTISTRY_IRI_HTTP || *self == DENTISTRY_IRI_HTTPS
	}
}
pub struct DentistryIriOrLabel;
impl PartialEq<&str> for DentistryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DentistryIri || *other == DENTISTRY_LABEL
	}
}
impl PartialEq<DentistryIriOrLabel> for &str {
	fn eq(&self, other: &DentistryIriOrLabel) -> bool {
		*self == DentistryIri || *self == DENTISTRY_LABEL
	}
}
