/// <https://schema.org/RestrictedDiet>
pub const RESTRICTED_DIET_IRI_HTTP: &str = "http://schema.org/RestrictedDiet";
/// <https://schema.org/RestrictedDiet>
pub const RESTRICTED_DIET_IRI_HTTPS: &str = "https://schema.org/RestrictedDiet";
/// <https://schema.org/RestrictedDiet>
pub const RESTRICTED_DIET_LABEL: &str = "RestrictedDiet";
pub struct RestrictedDietIri;
impl PartialEq<&str> for RestrictedDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESTRICTED_DIET_IRI_HTTP || *other == RESTRICTED_DIET_IRI_HTTPS
	}
}
impl PartialEq<RestrictedDietIri> for &str {
	fn eq(&self, other: &RestrictedDietIri) -> bool {
		*self == RESTRICTED_DIET_IRI_HTTP || *self == RESTRICTED_DIET_IRI_HTTPS
	}
}
pub struct RestrictedDietIriOrLabel;
impl PartialEq<&str> for RestrictedDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RestrictedDietIri || *other == RESTRICTED_DIET_LABEL
	}
}
impl PartialEq<RestrictedDietIriOrLabel> for &str {
	fn eq(&self, other: &RestrictedDietIriOrLabel) -> bool {
		*self == RestrictedDietIri || *self == RESTRICTED_DIET_LABEL
	}
}
