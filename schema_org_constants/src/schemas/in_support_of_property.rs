/// <https://schema.org/inSupportOf>
pub const IN_SUPPORT_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/inSupportOf";
/// <https://schema.org/inSupportOf>
pub const IN_SUPPORT_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inSupportOf";
/// <https://schema.org/inSupportOf>
pub const IN_SUPPORT_OF_PROPERTY_LABEL: &str = "inSupportOf";
pub struct InSupportOfPropertyIri;
impl PartialEq<&str> for InSupportOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_SUPPORT_OF_PROPERTY_IRI_HTTP || *other == IN_SUPPORT_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InSupportOfPropertyIri> for &str {
	fn eq(&self, other: &InSupportOfPropertyIri) -> bool {
		*self == IN_SUPPORT_OF_PROPERTY_IRI_HTTP || *self == IN_SUPPORT_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct InSupportOfPropertyIriOrLabel;
impl PartialEq<&str> for InSupportOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InSupportOfPropertyIri || *other == IN_SUPPORT_OF_PROPERTY_LABEL
	}
}
impl PartialEq<InSupportOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InSupportOfPropertyIriOrLabel) -> bool {
		*self == InSupportOfPropertyIri || *self == IN_SUPPORT_OF_PROPERTY_LABEL
	}
}
