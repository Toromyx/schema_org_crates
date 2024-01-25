/// <https://schema.org/jurisdiction>
pub const JURISDICTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/jurisdiction";
/// <https://schema.org/jurisdiction>
pub const JURISDICTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/jurisdiction";
/// <https://schema.org/jurisdiction>
pub const JURISDICTION_PROPERTY_LABEL: &str = "jurisdiction";
pub struct JurisdictionPropertyIri;
impl PartialEq<&str> for JurisdictionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JURISDICTION_PROPERTY_IRI_HTTP || *other == JURISDICTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<JurisdictionPropertyIri> for &str {
	fn eq(&self, other: &JurisdictionPropertyIri) -> bool {
		*self == JURISDICTION_PROPERTY_IRI_HTTP || *self == JURISDICTION_PROPERTY_IRI_HTTPS
	}
}
pub struct JurisdictionPropertyIriOrLabel;
impl PartialEq<&str> for JurisdictionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JurisdictionPropertyIri || *other == JURISDICTION_PROPERTY_LABEL
	}
}
impl PartialEq<JurisdictionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &JurisdictionPropertyIriOrLabel) -> bool {
		*self == JurisdictionPropertyIri || *self == JURISDICTION_PROPERTY_LABEL
	}
}
