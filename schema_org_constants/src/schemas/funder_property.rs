/// <https://schema.org/funder>
pub const FUNDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/funder";
/// <https://schema.org/funder>
pub const FUNDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/funder";
/// <https://schema.org/funder>
pub const FUNDER_PROPERTY_LABEL: &str = "funder";
pub struct FunderPropertyIri;
impl PartialEq<&str> for FunderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUNDER_PROPERTY_IRI_HTTP || *other == FUNDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FunderPropertyIri> for &str {
	fn eq(&self, other: &FunderPropertyIri) -> bool {
		*self == FUNDER_PROPERTY_IRI_HTTP || *self == FUNDER_PROPERTY_IRI_HTTPS
	}
}
pub struct FunderPropertyIriOrLabel;
impl PartialEq<&str> for FunderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FunderPropertyIri || *other == FUNDER_PROPERTY_LABEL
	}
}
impl PartialEq<FunderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FunderPropertyIriOrLabel) -> bool {
		*self == FunderPropertyIri || *self == FUNDER_PROPERTY_LABEL
	}
}
