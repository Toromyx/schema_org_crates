/// <https://schema.org/GovernmentOffice>
pub const GOVERNMENT_OFFICE_IRI_HTTP: &str = "http://schema.org/GovernmentOffice";
/// <https://schema.org/GovernmentOffice>
pub const GOVERNMENT_OFFICE_IRI_HTTPS: &str = "https://schema.org/GovernmentOffice";
/// <https://schema.org/GovernmentOffice>
pub const GOVERNMENT_OFFICE_LABEL: &str = "GovernmentOffice";
pub struct GovernmentOfficeIri;
impl PartialEq<&str> for GovernmentOfficeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GOVERNMENT_OFFICE_IRI_HTTP || *other == GOVERNMENT_OFFICE_IRI_HTTPS
	}
}
impl PartialEq<GovernmentOfficeIri> for &str {
	fn eq(&self, other: &GovernmentOfficeIri) -> bool {
		*self == GOVERNMENT_OFFICE_IRI_HTTP || *self == GOVERNMENT_OFFICE_IRI_HTTPS
	}
}
pub struct GovernmentOfficeIriOrLabel;
impl PartialEq<&str> for GovernmentOfficeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GovernmentOfficeIri || *other == GOVERNMENT_OFFICE_LABEL
	}
}
impl PartialEq<GovernmentOfficeIriOrLabel> for &str {
	fn eq(&self, other: &GovernmentOfficeIriOrLabel) -> bool {
		*self == GovernmentOfficeIri || *self == GOVERNMENT_OFFICE_LABEL
	}
}
