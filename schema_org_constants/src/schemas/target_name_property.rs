/// <https://schema.org/targetName>
pub const TARGET_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/targetName";
/// <https://schema.org/targetName>
pub const TARGET_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/targetName";
/// <https://schema.org/targetName>
pub const TARGET_NAME_PROPERTY_LABEL: &str = "targetName";
pub struct TargetNamePropertyIri;
impl PartialEq<&str> for TargetNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TARGET_NAME_PROPERTY_IRI_HTTP || *other == TARGET_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TargetNamePropertyIri> for &str {
	fn eq(&self, other: &TargetNamePropertyIri) -> bool {
		*self == TARGET_NAME_PROPERTY_IRI_HTTP || *self == TARGET_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct TargetNamePropertyIriOrLabel;
impl PartialEq<&str> for TargetNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TargetNamePropertyIri || *other == TARGET_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<TargetNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TargetNamePropertyIriOrLabel) -> bool {
		*self == TargetNamePropertyIri || *self == TARGET_NAME_PROPERTY_LABEL
	}
}
