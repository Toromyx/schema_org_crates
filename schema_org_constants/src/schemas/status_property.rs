/// <https://schema.org/status>
pub const STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/status";
/// <https://schema.org/status>
pub const STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/status";
/// <https://schema.org/status>
pub const STATUS_PROPERTY_LABEL: &str = "status";
pub struct StatusPropertyIri;
impl PartialEq<&str> for StatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STATUS_PROPERTY_IRI_HTTP || *other == STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StatusPropertyIri> for &str {
	fn eq(&self, other: &StatusPropertyIri) -> bool {
		*self == STATUS_PROPERTY_IRI_HTTP || *self == STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct StatusPropertyIriOrLabel;
impl PartialEq<&str> for StatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StatusPropertyIri || *other == STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<StatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StatusPropertyIriOrLabel) -> bool {
		*self == StatusPropertyIri || *self == STATUS_PROPERTY_LABEL
	}
}
