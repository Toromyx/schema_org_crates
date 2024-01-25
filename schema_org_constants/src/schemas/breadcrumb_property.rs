/// <https://schema.org/breadcrumb>
pub const BREADCRUMB_PROPERTY_IRI_HTTP: &str = "http://schema.org/breadcrumb";
/// <https://schema.org/breadcrumb>
pub const BREADCRUMB_PROPERTY_IRI_HTTPS: &str = "https://schema.org/breadcrumb";
/// <https://schema.org/breadcrumb>
pub const BREADCRUMB_PROPERTY_LABEL: &str = "breadcrumb";
pub struct BreadcrumbPropertyIri;
impl PartialEq<&str> for BreadcrumbPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BREADCRUMB_PROPERTY_IRI_HTTP || *other == BREADCRUMB_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BreadcrumbPropertyIri> for &str {
	fn eq(&self, other: &BreadcrumbPropertyIri) -> bool {
		*self == BREADCRUMB_PROPERTY_IRI_HTTP || *self == BREADCRUMB_PROPERTY_IRI_HTTPS
	}
}
pub struct BreadcrumbPropertyIriOrLabel;
impl PartialEq<&str> for BreadcrumbPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BreadcrumbPropertyIri || *other == BREADCRUMB_PROPERTY_LABEL
	}
}
impl PartialEq<BreadcrumbPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BreadcrumbPropertyIriOrLabel) -> bool {
		*self == BreadcrumbPropertyIri || *self == BREADCRUMB_PROPERTY_LABEL
	}
}
