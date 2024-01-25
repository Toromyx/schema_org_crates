/// <https://schema.org/primaryPrevention>
pub const PRIMARY_PREVENTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/primaryPrevention";
/// <https://schema.org/primaryPrevention>
pub const PRIMARY_PREVENTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/primaryPrevention";
/// <https://schema.org/primaryPrevention>
pub const PRIMARY_PREVENTION_PROPERTY_LABEL: &str = "primaryPrevention";
pub struct PrimaryPreventionPropertyIri;
impl PartialEq<&str> for PrimaryPreventionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRIMARY_PREVENTION_PROPERTY_IRI_HTTP
			|| *other == PRIMARY_PREVENTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PrimaryPreventionPropertyIri> for &str {
	fn eq(&self, other: &PrimaryPreventionPropertyIri) -> bool {
		*self == PRIMARY_PREVENTION_PROPERTY_IRI_HTTP
			|| *self == PRIMARY_PREVENTION_PROPERTY_IRI_HTTPS
	}
}
pub struct PrimaryPreventionPropertyIriOrLabel;
impl PartialEq<&str> for PrimaryPreventionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrimaryPreventionPropertyIri || *other == PRIMARY_PREVENTION_PROPERTY_LABEL
	}
}
impl PartialEq<PrimaryPreventionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PrimaryPreventionPropertyIriOrLabel) -> bool {
		*self == PrimaryPreventionPropertyIri || *self == PRIMARY_PREVENTION_PROPERTY_LABEL
	}
}
