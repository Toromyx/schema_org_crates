/// <https://schema.org/area>
#[deprecated = "This schema is superseded by <https://schema.org/serviceArea>."]
pub const AREA_PROPERTY_IRI_HTTP: &str = "http://schema.org/area";
/// <https://schema.org/area>
#[deprecated = "This schema is superseded by <https://schema.org/serviceArea>."]
pub const AREA_PROPERTY_IRI_HTTPS: &str = "https://schema.org/area";
/// <https://schema.org/area>
#[deprecated = "This schema is superseded by <https://schema.org/serviceArea>."]
pub const AREA_PROPERTY_LABEL: &str = "area";
pub struct AreaPropertyIri;
impl PartialEq<&str> for AreaPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AREA_PROPERTY_IRI_HTTP || *other == AREA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AreaPropertyIri> for &str {
	fn eq(&self, other: &AreaPropertyIri) -> bool {
		*self == AREA_PROPERTY_IRI_HTTP || *self == AREA_PROPERTY_IRI_HTTPS
	}
}
pub struct AreaPropertyIriOrLabel;
impl PartialEq<&str> for AreaPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AreaPropertyIri || *other == AREA_PROPERTY_LABEL
	}
}
impl PartialEq<AreaPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AreaPropertyIriOrLabel) -> bool {
		*self == AreaPropertyIri || *self == AREA_PROPERTY_LABEL
	}
}
