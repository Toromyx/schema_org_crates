/// <https://schema.org/AnatomicalSystem>
pub const ANATOMICAL_SYSTEM_IRI_HTTP: &str = "http://schema.org/AnatomicalSystem";
/// <https://schema.org/AnatomicalSystem>
pub const ANATOMICAL_SYSTEM_IRI_HTTPS: &str = "https://schema.org/AnatomicalSystem";
/// <https://schema.org/AnatomicalSystem>
pub const ANATOMICAL_SYSTEM_LABEL: &str = "AnatomicalSystem";
pub struct AnatomicalSystemIri;
impl PartialEq<&str> for AnatomicalSystemIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANATOMICAL_SYSTEM_IRI_HTTP || *other == ANATOMICAL_SYSTEM_IRI_HTTPS
	}
}
impl PartialEq<AnatomicalSystemIri> for &str {
	fn eq(&self, other: &AnatomicalSystemIri) -> bool {
		*self == ANATOMICAL_SYSTEM_IRI_HTTP || *self == ANATOMICAL_SYSTEM_IRI_HTTPS
	}
}
pub struct AnatomicalSystemIriOrLabel;
impl PartialEq<&str> for AnatomicalSystemIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnatomicalSystemIri || *other == ANATOMICAL_SYSTEM_LABEL
	}
}
impl PartialEq<AnatomicalSystemIriOrLabel> for &str {
	fn eq(&self, other: &AnatomicalSystemIriOrLabel) -> bool {
		*self == AnatomicalSystemIri || *self == ANATOMICAL_SYSTEM_LABEL
	}
}
