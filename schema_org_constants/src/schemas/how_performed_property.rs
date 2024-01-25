/// <https://schema.org/howPerformed>
pub const HOW_PERFORMED_PROPERTY_IRI_HTTP: &str = "http://schema.org/howPerformed";
/// <https://schema.org/howPerformed>
pub const HOW_PERFORMED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/howPerformed";
/// <https://schema.org/howPerformed>
pub const HOW_PERFORMED_PROPERTY_LABEL: &str = "howPerformed";
pub struct HowPerformedPropertyIri;
impl PartialEq<&str> for HowPerformedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_PERFORMED_PROPERTY_IRI_HTTP || *other == HOW_PERFORMED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HowPerformedPropertyIri> for &str {
	fn eq(&self, other: &HowPerformedPropertyIri) -> bool {
		*self == HOW_PERFORMED_PROPERTY_IRI_HTTP || *self == HOW_PERFORMED_PROPERTY_IRI_HTTPS
	}
}
pub struct HowPerformedPropertyIriOrLabel;
impl PartialEq<&str> for HowPerformedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowPerformedPropertyIri || *other == HOW_PERFORMED_PROPERTY_LABEL
	}
}
impl PartialEq<HowPerformedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HowPerformedPropertyIriOrLabel) -> bool {
		*self == HowPerformedPropertyIri || *self == HOW_PERFORMED_PROPERTY_LABEL
	}
}
