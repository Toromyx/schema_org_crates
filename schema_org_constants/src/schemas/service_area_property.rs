/// <https://schema.org/serviceArea>
#[deprecated = "This schema is superseded by <https://schema.org/areaServed>."]
pub const SERVICE_AREA_PROPERTY_IRI_HTTP: &str = "http://schema.org/serviceArea";
/// <https://schema.org/serviceArea>
#[deprecated = "This schema is superseded by <https://schema.org/areaServed>."]
pub const SERVICE_AREA_PROPERTY_IRI_HTTPS: &str = "https://schema.org/serviceArea";
/// <https://schema.org/serviceArea>
#[deprecated = "This schema is superseded by <https://schema.org/areaServed>."]
pub const SERVICE_AREA_PROPERTY_LABEL: &str = "serviceArea";
pub struct ServiceAreaPropertyIri;
impl PartialEq<&str> for ServiceAreaPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_AREA_PROPERTY_IRI_HTTP || *other == SERVICE_AREA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServiceAreaPropertyIri> for &str {
	fn eq(&self, other: &ServiceAreaPropertyIri) -> bool {
		*self == SERVICE_AREA_PROPERTY_IRI_HTTP || *self == SERVICE_AREA_PROPERTY_IRI_HTTPS
	}
}
pub struct ServiceAreaPropertyIriOrLabel;
impl PartialEq<&str> for ServiceAreaPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServiceAreaPropertyIri || *other == SERVICE_AREA_PROPERTY_LABEL
	}
}
impl PartialEq<ServiceAreaPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServiceAreaPropertyIriOrLabel) -> bool {
		*self == ServiceAreaPropertyIri || *self == SERVICE_AREA_PROPERTY_LABEL
	}
}
