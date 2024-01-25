/// <https://schema.org/Electrician>
pub const ELECTRICIAN_IRI_HTTP: &str = "http://schema.org/Electrician";
/// <https://schema.org/Electrician>
pub const ELECTRICIAN_IRI_HTTPS: &str = "https://schema.org/Electrician";
/// <https://schema.org/Electrician>
pub const ELECTRICIAN_LABEL: &str = "Electrician";
pub struct ElectricianIri;
impl PartialEq<&str> for ElectricianIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ELECTRICIAN_IRI_HTTP || *other == ELECTRICIAN_IRI_HTTPS
	}
}
impl PartialEq<ElectricianIri> for &str {
	fn eq(&self, other: &ElectricianIri) -> bool {
		*self == ELECTRICIAN_IRI_HTTP || *self == ELECTRICIAN_IRI_HTTPS
	}
}
pub struct ElectricianIriOrLabel;
impl PartialEq<&str> for ElectricianIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ElectricianIri || *other == ELECTRICIAN_LABEL
	}
}
impl PartialEq<ElectricianIriOrLabel> for &str {
	fn eq(&self, other: &ElectricianIriOrLabel) -> bool {
		*self == ElectricianIri || *self == ELECTRICIAN_LABEL
	}
}
