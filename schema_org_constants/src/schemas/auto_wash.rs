/// <https://schema.org/AutoWash>
pub const AUTO_WASH_IRI_HTTP: &str = "http://schema.org/AutoWash";
/// <https://schema.org/AutoWash>
pub const AUTO_WASH_IRI_HTTPS: &str = "https://schema.org/AutoWash";
/// <https://schema.org/AutoWash>
pub const AUTO_WASH_LABEL: &str = "AutoWash";
pub struct AutoWashIri;
impl PartialEq<&str> for AutoWashIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTO_WASH_IRI_HTTP || *other == AUTO_WASH_IRI_HTTPS
	}
}
impl PartialEq<AutoWashIri> for &str {
	fn eq(&self, other: &AutoWashIri) -> bool {
		*self == AUTO_WASH_IRI_HTTP || *self == AUTO_WASH_IRI_HTTPS
	}
}
pub struct AutoWashIriOrLabel;
impl PartialEq<&str> for AutoWashIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AutoWashIri || *other == AUTO_WASH_LABEL
	}
}
impl PartialEq<AutoWashIriOrLabel> for &str {
	fn eq(&self, other: &AutoWashIriOrLabel) -> bool {
		*self == AutoWashIri || *self == AUTO_WASH_LABEL
	}
}
