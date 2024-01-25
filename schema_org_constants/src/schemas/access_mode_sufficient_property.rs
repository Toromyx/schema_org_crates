/// <https://schema.org/accessModeSufficient>
pub const ACCESS_MODE_SUFFICIENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/accessModeSufficient";
/// <https://schema.org/accessModeSufficient>
pub const ACCESS_MODE_SUFFICIENT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/accessModeSufficient";
/// <https://schema.org/accessModeSufficient>
pub const ACCESS_MODE_SUFFICIENT_PROPERTY_LABEL: &str = "accessModeSufficient";
pub struct AccessModeSufficientPropertyIri;
impl PartialEq<&str> for AccessModeSufficientPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCESS_MODE_SUFFICIENT_PROPERTY_IRI_HTTP
			|| *other == ACCESS_MODE_SUFFICIENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccessModeSufficientPropertyIri> for &str {
	fn eq(&self, other: &AccessModeSufficientPropertyIri) -> bool {
		*self == ACCESS_MODE_SUFFICIENT_PROPERTY_IRI_HTTP
			|| *self == ACCESS_MODE_SUFFICIENT_PROPERTY_IRI_HTTPS
	}
}
pub struct AccessModeSufficientPropertyIriOrLabel;
impl PartialEq<&str> for AccessModeSufficientPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccessModeSufficientPropertyIri || *other == ACCESS_MODE_SUFFICIENT_PROPERTY_LABEL
	}
}
impl PartialEq<AccessModeSufficientPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccessModeSufficientPropertyIriOrLabel) -> bool {
		*self == AccessModeSufficientPropertyIri || *self == ACCESS_MODE_SUFFICIENT_PROPERTY_LABEL
	}
}
