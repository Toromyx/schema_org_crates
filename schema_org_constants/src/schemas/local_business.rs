/// <https://schema.org/LocalBusiness>
pub const LOCAL_BUSINESS_IRI_HTTP: &str = "http://schema.org/LocalBusiness";
/// <https://schema.org/LocalBusiness>
pub const LOCAL_BUSINESS_IRI_HTTPS: &str = "https://schema.org/LocalBusiness";
/// <https://schema.org/LocalBusiness>
pub const LOCAL_BUSINESS_LABEL: &str = "LocalBusiness";
pub struct LocalBusinessIri;
impl PartialEq<&str> for LocalBusinessIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOCAL_BUSINESS_IRI_HTTP || *other == LOCAL_BUSINESS_IRI_HTTPS
	}
}
impl PartialEq<LocalBusinessIri> for &str {
	fn eq(&self, other: &LocalBusinessIri) -> bool {
		*self == LOCAL_BUSINESS_IRI_HTTP || *self == LOCAL_BUSINESS_IRI_HTTPS
	}
}
pub struct LocalBusinessIriOrLabel;
impl PartialEq<&str> for LocalBusinessIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LocalBusinessIri || *other == LOCAL_BUSINESS_LABEL
	}
}
impl PartialEq<LocalBusinessIriOrLabel> for &str {
	fn eq(&self, other: &LocalBusinessIriOrLabel) -> bool {
		*self == LocalBusinessIri || *self == LOCAL_BUSINESS_LABEL
	}
}
