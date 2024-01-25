/// <https://schema.org/DepartmentStore>
pub const DEPARTMENT_STORE_IRI_HTTP: &str = "http://schema.org/DepartmentStore";
/// <https://schema.org/DepartmentStore>
pub const DEPARTMENT_STORE_IRI_HTTPS: &str = "https://schema.org/DepartmentStore";
/// <https://schema.org/DepartmentStore>
pub const DEPARTMENT_STORE_LABEL: &str = "DepartmentStore";
pub struct DepartmentStoreIri;
impl PartialEq<&str> for DepartmentStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPARTMENT_STORE_IRI_HTTP || *other == DEPARTMENT_STORE_IRI_HTTPS
	}
}
impl PartialEq<DepartmentStoreIri> for &str {
	fn eq(&self, other: &DepartmentStoreIri) -> bool {
		*self == DEPARTMENT_STORE_IRI_HTTP || *self == DEPARTMENT_STORE_IRI_HTTPS
	}
}
pub struct DepartmentStoreIriOrLabel;
impl PartialEq<&str> for DepartmentStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepartmentStoreIri || *other == DEPARTMENT_STORE_LABEL
	}
}
impl PartialEq<DepartmentStoreIriOrLabel> for &str {
	fn eq(&self, other: &DepartmentStoreIriOrLabel) -> bool {
		*self == DepartmentStoreIri || *self == DEPARTMENT_STORE_LABEL
	}
}
