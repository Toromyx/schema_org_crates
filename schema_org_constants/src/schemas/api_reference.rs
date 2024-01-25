/// <https://schema.org/APIReference>
pub const API_REFERENCE_IRI_HTTP: &str = "http://schema.org/APIReference";
/// <https://schema.org/APIReference>
pub const API_REFERENCE_IRI_HTTPS: &str = "https://schema.org/APIReference";
/// <https://schema.org/APIReference>
pub const API_REFERENCE_LABEL: &str = "APIReference";
pub struct ApiReferenceIri;
impl PartialEq<&str> for ApiReferenceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == API_REFERENCE_IRI_HTTP || *other == API_REFERENCE_IRI_HTTPS
	}
}
impl PartialEq<ApiReferenceIri> for &str {
	fn eq(&self, other: &ApiReferenceIri) -> bool {
		*self == API_REFERENCE_IRI_HTTP || *self == API_REFERENCE_IRI_HTTPS
	}
}
pub struct ApiReferenceIriOrLabel;
impl PartialEq<&str> for ApiReferenceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApiReferenceIri || *other == API_REFERENCE_LABEL
	}
}
impl PartialEq<ApiReferenceIriOrLabel> for &str {
	fn eq(&self, other: &ApiReferenceIriOrLabel) -> bool {
		*self == ApiReferenceIri || *self == API_REFERENCE_LABEL
	}
}
