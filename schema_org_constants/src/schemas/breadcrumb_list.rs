/// <https://schema.org/BreadcrumbList>
pub const BREADCRUMB_LIST_IRI_HTTP: &str = "http://schema.org/BreadcrumbList";
/// <https://schema.org/BreadcrumbList>
pub const BREADCRUMB_LIST_IRI_HTTPS: &str = "https://schema.org/BreadcrumbList";
/// <https://schema.org/BreadcrumbList>
pub const BREADCRUMB_LIST_LABEL: &str = "BreadcrumbList";
pub struct BreadcrumbListIri;
impl PartialEq<&str> for BreadcrumbListIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BREADCRUMB_LIST_IRI_HTTP || *other == BREADCRUMB_LIST_IRI_HTTPS
	}
}
impl PartialEq<BreadcrumbListIri> for &str {
	fn eq(&self, other: &BreadcrumbListIri) -> bool {
		*self == BREADCRUMB_LIST_IRI_HTTP || *self == BREADCRUMB_LIST_IRI_HTTPS
	}
}
pub struct BreadcrumbListIriOrLabel;
impl PartialEq<&str> for BreadcrumbListIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BreadcrumbListIri || *other == BREADCRUMB_LIST_LABEL
	}
}
impl PartialEq<BreadcrumbListIriOrLabel> for &str {
	fn eq(&self, other: &BreadcrumbListIriOrLabel) -> bool {
		*self == BreadcrumbListIri || *self == BREADCRUMB_LIST_LABEL
	}
}
