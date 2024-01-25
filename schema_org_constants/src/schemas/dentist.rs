/// <https://schema.org/Dentist>
pub const DENTIST_IRI_HTTP: &str = "http://schema.org/Dentist";
/// <https://schema.org/Dentist>
pub const DENTIST_IRI_HTTPS: &str = "https://schema.org/Dentist";
/// <https://schema.org/Dentist>
pub const DENTIST_LABEL: &str = "Dentist";
pub struct DentistIri;
impl PartialEq<&str> for DentistIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DENTIST_IRI_HTTP || *other == DENTIST_IRI_HTTPS
	}
}
impl PartialEq<DentistIri> for &str {
	fn eq(&self, other: &DentistIri) -> bool {
		*self == DENTIST_IRI_HTTP || *self == DENTIST_IRI_HTTPS
	}
}
pub struct DentistIriOrLabel;
impl PartialEq<&str> for DentistIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DentistIri || *other == DENTIST_LABEL
	}
}
impl PartialEq<DentistIriOrLabel> for &str {
	fn eq(&self, other: &DentistIriOrLabel) -> bool {
		*self == DentistIri || *self == DENTIST_LABEL
	}
}
