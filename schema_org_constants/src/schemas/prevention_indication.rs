/// <https://schema.org/PreventionIndication>
pub const PREVENTION_INDICATION_IRI_HTTP: &str = "http://schema.org/PreventionIndication";
/// <https://schema.org/PreventionIndication>
pub const PREVENTION_INDICATION_IRI_HTTPS: &str = "https://schema.org/PreventionIndication";
/// <https://schema.org/PreventionIndication>
pub const PREVENTION_INDICATION_LABEL: &str = "PreventionIndication";
pub struct PreventionIndicationIri;
impl PartialEq<&str> for PreventionIndicationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREVENTION_INDICATION_IRI_HTTP || *other == PREVENTION_INDICATION_IRI_HTTPS
	}
}
impl PartialEq<PreventionIndicationIri> for &str {
	fn eq(&self, other: &PreventionIndicationIri) -> bool {
		*self == PREVENTION_INDICATION_IRI_HTTP || *self == PREVENTION_INDICATION_IRI_HTTPS
	}
}
pub struct PreventionIndicationIriOrLabel;
impl PartialEq<&str> for PreventionIndicationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PreventionIndicationIri || *other == PREVENTION_INDICATION_LABEL
	}
}
impl PartialEq<PreventionIndicationIriOrLabel> for &str {
	fn eq(&self, other: &PreventionIndicationIriOrLabel) -> bool {
		*self == PreventionIndicationIri || *self == PREVENTION_INDICATION_LABEL
	}
}
