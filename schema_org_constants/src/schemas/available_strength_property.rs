/// <https://schema.org/availableStrength>
pub const AVAILABLE_STRENGTH_PROPERTY_IRI_HTTP: &str = "http://schema.org/availableStrength";
/// <https://schema.org/availableStrength>
pub const AVAILABLE_STRENGTH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availableStrength";
/// <https://schema.org/availableStrength>
pub const AVAILABLE_STRENGTH_PROPERTY_LABEL: &str = "availableStrength";
pub struct AvailableStrengthPropertyIri;
impl PartialEq<&str> for AvailableStrengthPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_STRENGTH_PROPERTY_IRI_HTTP
			|| *other == AVAILABLE_STRENGTH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableStrengthPropertyIri> for &str {
	fn eq(&self, other: &AvailableStrengthPropertyIri) -> bool {
		*self == AVAILABLE_STRENGTH_PROPERTY_IRI_HTTP
			|| *self == AVAILABLE_STRENGTH_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableStrengthPropertyIriOrLabel;
impl PartialEq<&str> for AvailableStrengthPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableStrengthPropertyIri || *other == AVAILABLE_STRENGTH_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableStrengthPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableStrengthPropertyIriOrLabel) -> bool {
		*self == AvailableStrengthPropertyIri || *self == AVAILABLE_STRENGTH_PROPERTY_LABEL
	}
}
