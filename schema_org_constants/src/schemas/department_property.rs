/// <https://schema.org/department>
pub const DEPARTMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/department";
/// <https://schema.org/department>
pub const DEPARTMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/department";
/// <https://schema.org/department>
pub const DEPARTMENT_PROPERTY_LABEL: &str = "department";
pub struct DepartmentPropertyIri;
impl PartialEq<&str> for DepartmentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPARTMENT_PROPERTY_IRI_HTTP || *other == DEPARTMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DepartmentPropertyIri> for &str {
	fn eq(&self, other: &DepartmentPropertyIri) -> bool {
		*self == DEPARTMENT_PROPERTY_IRI_HTTP || *self == DEPARTMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct DepartmentPropertyIriOrLabel;
impl PartialEq<&str> for DepartmentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepartmentPropertyIri || *other == DEPARTMENT_PROPERTY_LABEL
	}
}
impl PartialEq<DepartmentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DepartmentPropertyIriOrLabel) -> bool {
		*self == DepartmentPropertyIri || *self == DEPARTMENT_PROPERTY_LABEL
	}
}
