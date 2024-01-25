/// <https://schema.org/pagination>
pub const PAGINATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/pagination";
/// <https://schema.org/pagination>
pub const PAGINATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/pagination";
/// <https://schema.org/pagination>
pub const PAGINATION_PROPERTY_LABEL: &str = "pagination";
pub struct PaginationPropertyIri;
impl PartialEq<&str> for PaginationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAGINATION_PROPERTY_IRI_HTTP || *other == PAGINATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PaginationPropertyIri> for &str {
	fn eq(&self, other: &PaginationPropertyIri) -> bool {
		*self == PAGINATION_PROPERTY_IRI_HTTP || *self == PAGINATION_PROPERTY_IRI_HTTPS
	}
}
pub struct PaginationPropertyIriOrLabel;
impl PartialEq<&str> for PaginationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaginationPropertyIri || *other == PAGINATION_PROPERTY_LABEL
	}
}
impl PartialEq<PaginationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PaginationPropertyIriOrLabel) -> bool {
		*self == PaginationPropertyIri || *self == PAGINATION_PROPERTY_LABEL
	}
}
