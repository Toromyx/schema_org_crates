/// <https://schema.org/free>
#[deprecated = "This schema is superseded by <https://schema.org/isAccessibleForFree>."]
pub const FREE_PROPERTY_IRI_HTTP: &str = "http://schema.org/free";
/// <https://schema.org/free>
#[deprecated = "This schema is superseded by <https://schema.org/isAccessibleForFree>."]
pub const FREE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/free";
/// <https://schema.org/free>
#[deprecated = "This schema is superseded by <https://schema.org/isAccessibleForFree>."]
pub const FREE_PROPERTY_LABEL: &str = "free";
pub struct FreePropertyIri;
impl PartialEq<&str> for FreePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FREE_PROPERTY_IRI_HTTP || *other == FREE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FreePropertyIri> for &str {
	fn eq(&self, other: &FreePropertyIri) -> bool {
		*self == FREE_PROPERTY_IRI_HTTP || *self == FREE_PROPERTY_IRI_HTTPS
	}
}
pub struct FreePropertyIriOrLabel;
impl PartialEq<&str> for FreePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FreePropertyIri || *other == FREE_PROPERTY_LABEL
	}
}
impl PartialEq<FreePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FreePropertyIriOrLabel) -> bool {
		*self == FreePropertyIri || *self == FREE_PROPERTY_LABEL
	}
}
