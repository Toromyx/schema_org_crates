/// <https://schema.org/target>
pub const TARGET_PROPERTY_IRI_HTTP: &str = "http://schema.org/target";
/// <https://schema.org/target>
pub const TARGET_PROPERTY_IRI_HTTPS: &str = "https://schema.org/target";
/// <https://schema.org/target>
pub const TARGET_PROPERTY_LABEL: &str = "target";
pub struct TargetPropertyIri;
impl PartialEq<&str> for TargetPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TARGET_PROPERTY_IRI_HTTP || *other == TARGET_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TargetPropertyIri> for &str {
	fn eq(&self, other: &TargetPropertyIri) -> bool {
		*self == TARGET_PROPERTY_IRI_HTTP || *self == TARGET_PROPERTY_IRI_HTTPS
	}
}
pub struct TargetPropertyIriOrLabel;
impl PartialEq<&str> for TargetPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TargetPropertyIri || *other == TARGET_PROPERTY_LABEL
	}
}
impl PartialEq<TargetPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TargetPropertyIriOrLabel) -> bool {
		*self == TargetPropertyIri || *self == TARGET_PROPERTY_LABEL
	}
}
