/// <https://schema.org/antagonist>
pub const ANTAGONIST_PROPERTY_IRI_HTTP: &str = "http://schema.org/antagonist";
/// <https://schema.org/antagonist>
pub const ANTAGONIST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/antagonist";
/// <https://schema.org/antagonist>
pub const ANTAGONIST_PROPERTY_LABEL: &str = "antagonist";
pub struct AntagonistPropertyIri;
impl PartialEq<&str> for AntagonistPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANTAGONIST_PROPERTY_IRI_HTTP || *other == ANTAGONIST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AntagonistPropertyIri> for &str {
	fn eq(&self, other: &AntagonistPropertyIri) -> bool {
		*self == ANTAGONIST_PROPERTY_IRI_HTTP || *self == ANTAGONIST_PROPERTY_IRI_HTTPS
	}
}
pub struct AntagonistPropertyIriOrLabel;
impl PartialEq<&str> for AntagonistPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AntagonistPropertyIri || *other == ANTAGONIST_PROPERTY_LABEL
	}
}
impl PartialEq<AntagonistPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AntagonistPropertyIriOrLabel) -> bool {
		*self == AntagonistPropertyIri || *self == ANTAGONIST_PROPERTY_LABEL
	}
}
