/// <https://schema.org/primaryImageOfPage>
pub const PRIMARY_IMAGE_OF_PAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/primaryImageOfPage";
/// <https://schema.org/primaryImageOfPage>
pub const PRIMARY_IMAGE_OF_PAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/primaryImageOfPage";
/// <https://schema.org/primaryImageOfPage>
pub const PRIMARY_IMAGE_OF_PAGE_PROPERTY_LABEL: &str = "primaryImageOfPage";
pub struct PrimaryImageOfPagePropertyIri;
impl PartialEq<&str> for PrimaryImageOfPagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRIMARY_IMAGE_OF_PAGE_PROPERTY_IRI_HTTP
			|| *other == PRIMARY_IMAGE_OF_PAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PrimaryImageOfPagePropertyIri> for &str {
	fn eq(&self, other: &PrimaryImageOfPagePropertyIri) -> bool {
		*self == PRIMARY_IMAGE_OF_PAGE_PROPERTY_IRI_HTTP
			|| *self == PRIMARY_IMAGE_OF_PAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct PrimaryImageOfPagePropertyIriOrLabel;
impl PartialEq<&str> for PrimaryImageOfPagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrimaryImageOfPagePropertyIri || *other == PRIMARY_IMAGE_OF_PAGE_PROPERTY_LABEL
	}
}
impl PartialEq<PrimaryImageOfPagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PrimaryImageOfPagePropertyIriOrLabel) -> bool {
		*self == PrimaryImageOfPagePropertyIri || *self == PRIMARY_IMAGE_OF_PAGE_PROPERTY_LABEL
	}
}
