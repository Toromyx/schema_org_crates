/// <https://schema.org/secondaryPrevention>
pub const SECONDARY_PREVENTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/secondaryPrevention";
/// <https://schema.org/secondaryPrevention>
pub const SECONDARY_PREVENTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/secondaryPrevention";
/// <https://schema.org/secondaryPrevention>
pub const SECONDARY_PREVENTION_PROPERTY_LABEL: &str = "secondaryPrevention";
pub struct SecondaryPreventionPropertyIri;
impl PartialEq<&str> for SecondaryPreventionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SECONDARY_PREVENTION_PROPERTY_IRI_HTTP
			|| *other == SECONDARY_PREVENTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SecondaryPreventionPropertyIri> for &str {
	fn eq(&self, other: &SecondaryPreventionPropertyIri) -> bool {
		*self == SECONDARY_PREVENTION_PROPERTY_IRI_HTTP
			|| *self == SECONDARY_PREVENTION_PROPERTY_IRI_HTTPS
	}
}
pub struct SecondaryPreventionPropertyIriOrLabel;
impl PartialEq<&str> for SecondaryPreventionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SecondaryPreventionPropertyIri || *other == SECONDARY_PREVENTION_PROPERTY_LABEL
	}
}
impl PartialEq<SecondaryPreventionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SecondaryPreventionPropertyIriOrLabel) -> bool {
		*self == SecondaryPreventionPropertyIri || *self == SECONDARY_PREVENTION_PROPERTY_LABEL
	}
}
