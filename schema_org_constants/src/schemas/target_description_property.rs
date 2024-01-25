/// <https://schema.org/targetDescription>
pub const TARGET_DESCRIPTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/targetDescription";
/// <https://schema.org/targetDescription>
pub const TARGET_DESCRIPTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/targetDescription";
/// <https://schema.org/targetDescription>
pub const TARGET_DESCRIPTION_PROPERTY_LABEL: &str = "targetDescription";
pub struct TargetDescriptionPropertyIri;
impl PartialEq<&str> for TargetDescriptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TARGET_DESCRIPTION_PROPERTY_IRI_HTTP
			|| *other == TARGET_DESCRIPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TargetDescriptionPropertyIri> for &str {
	fn eq(&self, other: &TargetDescriptionPropertyIri) -> bool {
		*self == TARGET_DESCRIPTION_PROPERTY_IRI_HTTP
			|| *self == TARGET_DESCRIPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct TargetDescriptionPropertyIriOrLabel;
impl PartialEq<&str> for TargetDescriptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TargetDescriptionPropertyIri || *other == TARGET_DESCRIPTION_PROPERTY_LABEL
	}
}
impl PartialEq<TargetDescriptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TargetDescriptionPropertyIriOrLabel) -> bool {
		*self == TargetDescriptionPropertyIri || *self == TARGET_DESCRIPTION_PROPERTY_LABEL
	}
}
