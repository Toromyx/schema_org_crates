/// <https://schema.org/regionsAllowed>
pub const REGIONS_ALLOWED_PROPERTY_IRI_HTTP: &str = "http://schema.org/regionsAllowed";
/// <https://schema.org/regionsAllowed>
pub const REGIONS_ALLOWED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/regionsAllowed";
/// <https://schema.org/regionsAllowed>
pub const REGIONS_ALLOWED_PROPERTY_LABEL: &str = "regionsAllowed";
pub struct RegionsAllowedPropertyIri;
impl PartialEq<&str> for RegionsAllowedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REGIONS_ALLOWED_PROPERTY_IRI_HTTP || *other == REGIONS_ALLOWED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RegionsAllowedPropertyIri> for &str {
	fn eq(&self, other: &RegionsAllowedPropertyIri) -> bool {
		*self == REGIONS_ALLOWED_PROPERTY_IRI_HTTP || *self == REGIONS_ALLOWED_PROPERTY_IRI_HTTPS
	}
}
pub struct RegionsAllowedPropertyIriOrLabel;
impl PartialEq<&str> for RegionsAllowedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RegionsAllowedPropertyIri || *other == REGIONS_ALLOWED_PROPERTY_LABEL
	}
}
impl PartialEq<RegionsAllowedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RegionsAllowedPropertyIriOrLabel) -> bool {
		*self == RegionsAllowedPropertyIri || *self == REGIONS_ALLOWED_PROPERTY_LABEL
	}
}
