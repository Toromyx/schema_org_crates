/// <https://schema.org/warning>
pub const WARNING_PROPERTY_IRI_HTTP: &str = "http://schema.org/warning";
/// <https://schema.org/warning>
pub const WARNING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/warning";
/// <https://schema.org/warning>
pub const WARNING_PROPERTY_LABEL: &str = "warning";
pub struct WarningPropertyIri;
impl PartialEq<&str> for WarningPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WARNING_PROPERTY_IRI_HTTP || *other == WARNING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WarningPropertyIri> for &str {
	fn eq(&self, other: &WarningPropertyIri) -> bool {
		*self == WARNING_PROPERTY_IRI_HTTP || *self == WARNING_PROPERTY_IRI_HTTPS
	}
}
pub struct WarningPropertyIriOrLabel;
impl PartialEq<&str> for WarningPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WarningPropertyIri || *other == WARNING_PROPERTY_LABEL
	}
}
impl PartialEq<WarningPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WarningPropertyIriOrLabel) -> bool {
		*self == WarningPropertyIri || *self == WARNING_PROPERTY_LABEL
	}
}
