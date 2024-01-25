/// <https://schema.org/HealthAspectEnumeration>
pub const HEALTH_ASPECT_ENUMERATION_IRI_HTTP: &str = "http://schema.org/HealthAspectEnumeration";
/// <https://schema.org/HealthAspectEnumeration>
pub const HEALTH_ASPECT_ENUMERATION_IRI_HTTPS: &str = "https://schema.org/HealthAspectEnumeration";
/// <https://schema.org/HealthAspectEnumeration>
pub const HEALTH_ASPECT_ENUMERATION_LABEL: &str = "HealthAspectEnumeration";
pub struct HealthAspectEnumerationIri;
impl PartialEq<&str> for HealthAspectEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_ASPECT_ENUMERATION_IRI_HTTP
			|| *other == HEALTH_ASPECT_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<HealthAspectEnumerationIri> for &str {
	fn eq(&self, other: &HealthAspectEnumerationIri) -> bool {
		*self == HEALTH_ASPECT_ENUMERATION_IRI_HTTP || *self == HEALTH_ASPECT_ENUMERATION_IRI_HTTPS
	}
}
pub struct HealthAspectEnumerationIriOrLabel;
impl PartialEq<&str> for HealthAspectEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthAspectEnumerationIri || *other == HEALTH_ASPECT_ENUMERATION_LABEL
	}
}
impl PartialEq<HealthAspectEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &HealthAspectEnumerationIriOrLabel) -> bool {
		*self == HealthAspectEnumerationIri || *self == HEALTH_ASPECT_ENUMERATION_LABEL
	}
}
