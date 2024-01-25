/// <https://schema.org/drainsTo>
pub const DRAINS_TO_PROPERTY_IRI_HTTP: &str = "http://schema.org/drainsTo";
/// <https://schema.org/drainsTo>
pub const DRAINS_TO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/drainsTo";
/// <https://schema.org/drainsTo>
pub const DRAINS_TO_PROPERTY_LABEL: &str = "drainsTo";
pub struct DrainsToPropertyIri;
impl PartialEq<&str> for DrainsToPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRAINS_TO_PROPERTY_IRI_HTTP || *other == DRAINS_TO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DrainsToPropertyIri> for &str {
	fn eq(&self, other: &DrainsToPropertyIri) -> bool {
		*self == DRAINS_TO_PROPERTY_IRI_HTTP || *self == DRAINS_TO_PROPERTY_IRI_HTTPS
	}
}
pub struct DrainsToPropertyIriOrLabel;
impl PartialEq<&str> for DrainsToPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrainsToPropertyIri || *other == DRAINS_TO_PROPERTY_LABEL
	}
}
impl PartialEq<DrainsToPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DrainsToPropertyIriOrLabel) -> bool {
		*self == DrainsToPropertyIri || *self == DRAINS_TO_PROPERTY_LABEL
	}
}
