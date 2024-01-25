/// <https://schema.org/administrationRoute>
pub const ADMINISTRATION_ROUTE_PROPERTY_IRI_HTTP: &str = "http://schema.org/administrationRoute";
/// <https://schema.org/administrationRoute>
pub const ADMINISTRATION_ROUTE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/administrationRoute";
/// <https://schema.org/administrationRoute>
pub const ADMINISTRATION_ROUTE_PROPERTY_LABEL: &str = "administrationRoute";
pub struct AdministrationRoutePropertyIri;
impl PartialEq<&str> for AdministrationRoutePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADMINISTRATION_ROUTE_PROPERTY_IRI_HTTP
			|| *other == ADMINISTRATION_ROUTE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AdministrationRoutePropertyIri> for &str {
	fn eq(&self, other: &AdministrationRoutePropertyIri) -> bool {
		*self == ADMINISTRATION_ROUTE_PROPERTY_IRI_HTTP
			|| *self == ADMINISTRATION_ROUTE_PROPERTY_IRI_HTTPS
	}
}
pub struct AdministrationRoutePropertyIriOrLabel;
impl PartialEq<&str> for AdministrationRoutePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdministrationRoutePropertyIri || *other == ADMINISTRATION_ROUTE_PROPERTY_LABEL
	}
}
impl PartialEq<AdministrationRoutePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AdministrationRoutePropertyIriOrLabel) -> bool {
		*self == AdministrationRoutePropertyIri || *self == ADMINISTRATION_ROUTE_PROPERTY_LABEL
	}
}
