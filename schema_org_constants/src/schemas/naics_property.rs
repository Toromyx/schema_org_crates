/// <https://schema.org/naics>
pub const NAICS_PROPERTY_IRI_HTTP: &str = "http://schema.org/naics";
/// <https://schema.org/naics>
pub const NAICS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/naics";
/// <https://schema.org/naics>
pub const NAICS_PROPERTY_LABEL: &str = "naics";
pub struct NaicsPropertyIri;
impl PartialEq<&str> for NaicsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NAICS_PROPERTY_IRI_HTTP || *other == NAICS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NaicsPropertyIri> for &str {
	fn eq(&self, other: &NaicsPropertyIri) -> bool {
		*self == NAICS_PROPERTY_IRI_HTTP || *self == NAICS_PROPERTY_IRI_HTTPS
	}
}
pub struct NaicsPropertyIriOrLabel;
impl PartialEq<&str> for NaicsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NaicsPropertyIri || *other == NAICS_PROPERTY_LABEL
	}
}
impl PartialEq<NaicsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NaicsPropertyIriOrLabel) -> bool {
		*self == NaicsPropertyIri || *self == NAICS_PROPERTY_LABEL
	}
}
