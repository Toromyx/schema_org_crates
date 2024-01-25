/// <https://schema.org/providesService>
pub const PROVIDES_SERVICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/providesService";
/// <https://schema.org/providesService>
pub const PROVIDES_SERVICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/providesService";
/// <https://schema.org/providesService>
pub const PROVIDES_SERVICE_PROPERTY_LABEL: &str = "providesService";
pub struct ProvidesServicePropertyIri;
impl PartialEq<&str> for ProvidesServicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROVIDES_SERVICE_PROPERTY_IRI_HTTP
			|| *other == PROVIDES_SERVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProvidesServicePropertyIri> for &str {
	fn eq(&self, other: &ProvidesServicePropertyIri) -> bool {
		*self == PROVIDES_SERVICE_PROPERTY_IRI_HTTP || *self == PROVIDES_SERVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct ProvidesServicePropertyIriOrLabel;
impl PartialEq<&str> for ProvidesServicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProvidesServicePropertyIri || *other == PROVIDES_SERVICE_PROPERTY_LABEL
	}
}
impl PartialEq<ProvidesServicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProvidesServicePropertyIriOrLabel) -> bool {
		*self == ProvidesServicePropertyIri || *self == PROVIDES_SERVICE_PROPERTY_LABEL
	}
}
