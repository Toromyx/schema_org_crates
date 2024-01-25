/// <https://schema.org/emissionsCO2>
pub const EMISSIONS_CO_2_PROPERTY_IRI_HTTP: &str = "http://schema.org/emissionsCO2";
/// <https://schema.org/emissionsCO2>
pub const EMISSIONS_CO_2_PROPERTY_IRI_HTTPS: &str = "https://schema.org/emissionsCO2";
/// <https://schema.org/emissionsCO2>
pub const EMISSIONS_CO_2_PROPERTY_LABEL: &str = "emissionsCO2";
pub struct EmissionsCo2PropertyIri;
impl PartialEq<&str> for EmissionsCo2PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMISSIONS_CO_2_PROPERTY_IRI_HTTP || *other == EMISSIONS_CO_2_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EmissionsCo2PropertyIri> for &str {
	fn eq(&self, other: &EmissionsCo2PropertyIri) -> bool {
		*self == EMISSIONS_CO_2_PROPERTY_IRI_HTTP || *self == EMISSIONS_CO_2_PROPERTY_IRI_HTTPS
	}
}
pub struct EmissionsCo2PropertyIriOrLabel;
impl PartialEq<&str> for EmissionsCo2PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmissionsCo2PropertyIri || *other == EMISSIONS_CO_2_PROPERTY_LABEL
	}
}
impl PartialEq<EmissionsCo2PropertyIriOrLabel> for &str {
	fn eq(&self, other: &EmissionsCo2PropertyIriOrLabel) -> bool {
		*self == EmissionsCo2PropertyIri || *self == EMISSIONS_CO_2_PROPERTY_LABEL
	}
}
