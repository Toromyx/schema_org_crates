/// <https://schema.org/isFamilyFriendly>
pub const IS_FAMILY_FRIENDLY_PROPERTY_IRI_HTTP: &str = "http://schema.org/isFamilyFriendly";
/// <https://schema.org/isFamilyFriendly>
pub const IS_FAMILY_FRIENDLY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isFamilyFriendly";
/// <https://schema.org/isFamilyFriendly>
pub const IS_FAMILY_FRIENDLY_PROPERTY_LABEL: &str = "isFamilyFriendly";
pub struct IsFamilyFriendlyPropertyIri;
impl PartialEq<&str> for IsFamilyFriendlyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_FAMILY_FRIENDLY_PROPERTY_IRI_HTTP
			|| *other == IS_FAMILY_FRIENDLY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsFamilyFriendlyPropertyIri> for &str {
	fn eq(&self, other: &IsFamilyFriendlyPropertyIri) -> bool {
		*self == IS_FAMILY_FRIENDLY_PROPERTY_IRI_HTTP
			|| *self == IS_FAMILY_FRIENDLY_PROPERTY_IRI_HTTPS
	}
}
pub struct IsFamilyFriendlyPropertyIriOrLabel;
impl PartialEq<&str> for IsFamilyFriendlyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsFamilyFriendlyPropertyIri || *other == IS_FAMILY_FRIENDLY_PROPERTY_LABEL
	}
}
impl PartialEq<IsFamilyFriendlyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsFamilyFriendlyPropertyIriOrLabel) -> bool {
		*self == IsFamilyFriendlyPropertyIri || *self == IS_FAMILY_FRIENDLY_PROPERTY_LABEL
	}
}
