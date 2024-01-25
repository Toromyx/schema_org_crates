/// <https://schema.org/targetUrl>
pub const TARGET_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/targetUrl";
/// <https://schema.org/targetUrl>
pub const TARGET_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/targetUrl";
/// <https://schema.org/targetUrl>
pub const TARGET_URL_PROPERTY_LABEL: &str = "targetUrl";
pub struct TargetUrlPropertyIri;
impl PartialEq<&str> for TargetUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TARGET_URL_PROPERTY_IRI_HTTP || *other == TARGET_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TargetUrlPropertyIri> for &str {
	fn eq(&self, other: &TargetUrlPropertyIri) -> bool {
		*self == TARGET_URL_PROPERTY_IRI_HTTP || *self == TARGET_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct TargetUrlPropertyIriOrLabel;
impl PartialEq<&str> for TargetUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TargetUrlPropertyIri || *other == TARGET_URL_PROPERTY_LABEL
	}
}
impl PartialEq<TargetUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TargetUrlPropertyIriOrLabel) -> bool {
		*self == TargetUrlPropertyIri || *self == TARGET_URL_PROPERTY_LABEL
	}
}
