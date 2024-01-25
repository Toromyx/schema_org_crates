/// <https://schema.org/targetProduct>
pub const TARGET_PRODUCT_PROPERTY_IRI_HTTP: &str = "http://schema.org/targetProduct";
/// <https://schema.org/targetProduct>
pub const TARGET_PRODUCT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/targetProduct";
/// <https://schema.org/targetProduct>
pub const TARGET_PRODUCT_PROPERTY_LABEL: &str = "targetProduct";
pub struct TargetProductPropertyIri;
impl PartialEq<&str> for TargetProductPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TARGET_PRODUCT_PROPERTY_IRI_HTTP || *other == TARGET_PRODUCT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TargetProductPropertyIri> for &str {
	fn eq(&self, other: &TargetProductPropertyIri) -> bool {
		*self == TARGET_PRODUCT_PROPERTY_IRI_HTTP || *self == TARGET_PRODUCT_PROPERTY_IRI_HTTPS
	}
}
pub struct TargetProductPropertyIriOrLabel;
impl PartialEq<&str> for TargetProductPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TargetProductPropertyIri || *other == TARGET_PRODUCT_PROPERTY_LABEL
	}
}
impl PartialEq<TargetProductPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TargetProductPropertyIriOrLabel) -> bool {
		*self == TargetProductPropertyIri || *self == TARGET_PRODUCT_PROPERTY_LABEL
	}
}
