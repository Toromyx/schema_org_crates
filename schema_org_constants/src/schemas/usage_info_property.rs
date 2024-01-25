/// <https://schema.org/usageInfo>
pub const USAGE_INFO_PROPERTY_IRI_HTTP: &str = "http://schema.org/usageInfo";
/// <https://schema.org/usageInfo>
pub const USAGE_INFO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/usageInfo";
/// <https://schema.org/usageInfo>
pub const USAGE_INFO_PROPERTY_LABEL: &str = "usageInfo";
pub struct UsageInfoPropertyIri;
impl PartialEq<&str> for UsageInfoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USAGE_INFO_PROPERTY_IRI_HTTP || *other == USAGE_INFO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UsageInfoPropertyIri> for &str {
	fn eq(&self, other: &UsageInfoPropertyIri) -> bool {
		*self == USAGE_INFO_PROPERTY_IRI_HTTP || *self == USAGE_INFO_PROPERTY_IRI_HTTPS
	}
}
pub struct UsageInfoPropertyIriOrLabel;
impl PartialEq<&str> for UsageInfoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UsageInfoPropertyIri || *other == USAGE_INFO_PROPERTY_LABEL
	}
}
impl PartialEq<UsageInfoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &UsageInfoPropertyIriOrLabel) -> bool {
		*self == UsageInfoPropertyIri || *self == USAGE_INFO_PROPERTY_LABEL
	}
}
