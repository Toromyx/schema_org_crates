/// <https://schema.org/meetsEmissionStandard>
pub const MEETS_EMISSION_STANDARD_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/meetsEmissionStandard";
/// <https://schema.org/meetsEmissionStandard>
pub const MEETS_EMISSION_STANDARD_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/meetsEmissionStandard";
/// <https://schema.org/meetsEmissionStandard>
pub const MEETS_EMISSION_STANDARD_PROPERTY_LABEL: &str = "meetsEmissionStandard";
pub struct MeetsEmissionStandardPropertyIri;
impl PartialEq<&str> for MeetsEmissionStandardPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEETS_EMISSION_STANDARD_PROPERTY_IRI_HTTP
			|| *other == MEETS_EMISSION_STANDARD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MeetsEmissionStandardPropertyIri> for &str {
	fn eq(&self, other: &MeetsEmissionStandardPropertyIri) -> bool {
		*self == MEETS_EMISSION_STANDARD_PROPERTY_IRI_HTTP
			|| *self == MEETS_EMISSION_STANDARD_PROPERTY_IRI_HTTPS
	}
}
pub struct MeetsEmissionStandardPropertyIriOrLabel;
impl PartialEq<&str> for MeetsEmissionStandardPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MeetsEmissionStandardPropertyIri
			|| *other == MEETS_EMISSION_STANDARD_PROPERTY_LABEL
	}
}
impl PartialEq<MeetsEmissionStandardPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MeetsEmissionStandardPropertyIriOrLabel) -> bool {
		*self == MeetsEmissionStandardPropertyIri || *self == MEETS_EMISSION_STANDARD_PROPERTY_LABEL
	}
}
