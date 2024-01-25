/// <https://schema.org/PhysiciansOffice>
pub const PHYSICIANS_OFFICE_IRI_HTTP: &str = "http://schema.org/PhysiciansOffice";
/// <https://schema.org/PhysiciansOffice>
pub const PHYSICIANS_OFFICE_IRI_HTTPS: &str = "https://schema.org/PhysiciansOffice";
/// <https://schema.org/PhysiciansOffice>
pub const PHYSICIANS_OFFICE_LABEL: &str = "PhysiciansOffice";
pub struct PhysiciansOfficeIri;
impl PartialEq<&str> for PhysiciansOfficeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHYSICIANS_OFFICE_IRI_HTTP || *other == PHYSICIANS_OFFICE_IRI_HTTPS
	}
}
impl PartialEq<PhysiciansOfficeIri> for &str {
	fn eq(&self, other: &PhysiciansOfficeIri) -> bool {
		*self == PHYSICIANS_OFFICE_IRI_HTTP || *self == PHYSICIANS_OFFICE_IRI_HTTPS
	}
}
pub struct PhysiciansOfficeIriOrLabel;
impl PartialEq<&str> for PhysiciansOfficeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhysiciansOfficeIri || *other == PHYSICIANS_OFFICE_LABEL
	}
}
impl PartialEq<PhysiciansOfficeIriOrLabel> for &str {
	fn eq(&self, other: &PhysiciansOfficeIriOrLabel) -> bool {
		*self == PhysiciansOfficeIri || *self == PHYSICIANS_OFFICE_LABEL
	}
}
