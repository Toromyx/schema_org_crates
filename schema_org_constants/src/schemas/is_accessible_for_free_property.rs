/// <https://schema.org/isAccessibleForFree>
pub const IS_ACCESSIBLE_FOR_FREE_PROPERTY_IRI_HTTP: &str = "http://schema.org/isAccessibleForFree";
/// <https://schema.org/isAccessibleForFree>
pub const IS_ACCESSIBLE_FOR_FREE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/isAccessibleForFree";
/// <https://schema.org/isAccessibleForFree>
pub const IS_ACCESSIBLE_FOR_FREE_PROPERTY_LABEL: &str = "isAccessibleForFree";
pub struct IsAccessibleForFreePropertyIri;
impl PartialEq<&str> for IsAccessibleForFreePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_ACCESSIBLE_FOR_FREE_PROPERTY_IRI_HTTP
			|| *other == IS_ACCESSIBLE_FOR_FREE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsAccessibleForFreePropertyIri> for &str {
	fn eq(&self, other: &IsAccessibleForFreePropertyIri) -> bool {
		*self == IS_ACCESSIBLE_FOR_FREE_PROPERTY_IRI_HTTP
			|| *self == IS_ACCESSIBLE_FOR_FREE_PROPERTY_IRI_HTTPS
	}
}
pub struct IsAccessibleForFreePropertyIriOrLabel;
impl PartialEq<&str> for IsAccessibleForFreePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsAccessibleForFreePropertyIri || *other == IS_ACCESSIBLE_FOR_FREE_PROPERTY_LABEL
	}
}
impl PartialEq<IsAccessibleForFreePropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsAccessibleForFreePropertyIriOrLabel) -> bool {
		*self == IsAccessibleForFreePropertyIri || *self == IS_ACCESSIBLE_FOR_FREE_PROPERTY_LABEL
	}
}
