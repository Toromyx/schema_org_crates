/// <https://schema.org/isicV4>
pub const ISIC_V_4_PROPERTY_IRI_HTTP: &str = "http://schema.org/isicV4";
/// <https://schema.org/isicV4>
pub const ISIC_V_4_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isicV4";
/// <https://schema.org/isicV4>
pub const ISIC_V_4_PROPERTY_LABEL: &str = "isicV4";
pub struct IsicV4PropertyIri;
impl PartialEq<&str> for IsicV4PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ISIC_V_4_PROPERTY_IRI_HTTP || *other == ISIC_V_4_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsicV4PropertyIri> for &str {
	fn eq(&self, other: &IsicV4PropertyIri) -> bool {
		*self == ISIC_V_4_PROPERTY_IRI_HTTP || *self == ISIC_V_4_PROPERTY_IRI_HTTPS
	}
}
pub struct IsicV4PropertyIriOrLabel;
impl PartialEq<&str> for IsicV4PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsicV4PropertyIri || *other == ISIC_V_4_PROPERTY_LABEL
	}
}
impl PartialEq<IsicV4PropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsicV4PropertyIriOrLabel) -> bool {
		*self == IsicV4PropertyIri || *self == ISIC_V_4_PROPERTY_LABEL
	}
}
