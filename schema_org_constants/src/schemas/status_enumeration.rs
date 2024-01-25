/// <https://schema.org/StatusEnumeration>
pub const STATUS_ENUMERATION_IRI_HTTP: &str = "http://schema.org/StatusEnumeration";
/// <https://schema.org/StatusEnumeration>
pub const STATUS_ENUMERATION_IRI_HTTPS: &str = "https://schema.org/StatusEnumeration";
/// <https://schema.org/StatusEnumeration>
pub const STATUS_ENUMERATION_LABEL: &str = "StatusEnumeration";
pub struct StatusEnumerationIri;
impl PartialEq<&str> for StatusEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STATUS_ENUMERATION_IRI_HTTP || *other == STATUS_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<StatusEnumerationIri> for &str {
	fn eq(&self, other: &StatusEnumerationIri) -> bool {
		*self == STATUS_ENUMERATION_IRI_HTTP || *self == STATUS_ENUMERATION_IRI_HTTPS
	}
}
pub struct StatusEnumerationIriOrLabel;
impl PartialEq<&str> for StatusEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StatusEnumerationIri || *other == STATUS_ENUMERATION_LABEL
	}
}
impl PartialEq<StatusEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &StatusEnumerationIriOrLabel) -> bool {
		*self == StatusEnumerationIri || *self == STATUS_ENUMERATION_LABEL
	}
}
