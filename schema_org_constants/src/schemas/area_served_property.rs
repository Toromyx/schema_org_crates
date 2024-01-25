/// <https://schema.org/areaServed>
pub const AREA_SERVED_PROPERTY_IRI_HTTP: &str = "http://schema.org/areaServed";
/// <https://schema.org/areaServed>
pub const AREA_SERVED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/areaServed";
/// <https://schema.org/areaServed>
pub const AREA_SERVED_PROPERTY_LABEL: &str = "areaServed";
pub struct AreaServedPropertyIri;
impl PartialEq<&str> for AreaServedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AREA_SERVED_PROPERTY_IRI_HTTP || *other == AREA_SERVED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AreaServedPropertyIri> for &str {
	fn eq(&self, other: &AreaServedPropertyIri) -> bool {
		*self == AREA_SERVED_PROPERTY_IRI_HTTP || *self == AREA_SERVED_PROPERTY_IRI_HTTPS
	}
}
pub struct AreaServedPropertyIriOrLabel;
impl PartialEq<&str> for AreaServedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AreaServedPropertyIri || *other == AREA_SERVED_PROPERTY_LABEL
	}
}
impl PartialEq<AreaServedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AreaServedPropertyIriOrLabel) -> bool {
		*self == AreaServedPropertyIri || *self == AREA_SERVED_PROPERTY_LABEL
	}
}
