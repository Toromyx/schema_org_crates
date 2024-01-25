/// <https://schema.org/enginePower>
pub const ENGINE_POWER_PROPERTY_IRI_HTTP: &str = "http://schema.org/enginePower";
/// <https://schema.org/enginePower>
pub const ENGINE_POWER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/enginePower";
/// <https://schema.org/enginePower>
pub const ENGINE_POWER_PROPERTY_LABEL: &str = "enginePower";
pub struct EnginePowerPropertyIri;
impl PartialEq<&str> for EnginePowerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENGINE_POWER_PROPERTY_IRI_HTTP || *other == ENGINE_POWER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EnginePowerPropertyIri> for &str {
	fn eq(&self, other: &EnginePowerPropertyIri) -> bool {
		*self == ENGINE_POWER_PROPERTY_IRI_HTTP || *self == ENGINE_POWER_PROPERTY_IRI_HTTPS
	}
}
pub struct EnginePowerPropertyIriOrLabel;
impl PartialEq<&str> for EnginePowerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EnginePowerPropertyIri || *other == ENGINE_POWER_PROPERTY_LABEL
	}
}
impl PartialEq<EnginePowerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EnginePowerPropertyIriOrLabel) -> bool {
		*self == EnginePowerPropertyIri || *self == ENGINE_POWER_PROPERTY_LABEL
	}
}
