/// <https://schema.org/isProprietary>
pub const IS_PROPRIETARY_PROPERTY_IRI_HTTP: &str = "http://schema.org/isProprietary";
/// <https://schema.org/isProprietary>
pub const IS_PROPRIETARY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isProprietary";
/// <https://schema.org/isProprietary>
pub const IS_PROPRIETARY_PROPERTY_LABEL: &str = "isProprietary";
pub struct IsProprietaryPropertyIri;
impl PartialEq<&str> for IsProprietaryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_PROPRIETARY_PROPERTY_IRI_HTTP || *other == IS_PROPRIETARY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsProprietaryPropertyIri> for &str {
	fn eq(&self, other: &IsProprietaryPropertyIri) -> bool {
		*self == IS_PROPRIETARY_PROPERTY_IRI_HTTP || *self == IS_PROPRIETARY_PROPERTY_IRI_HTTPS
	}
}
pub struct IsProprietaryPropertyIriOrLabel;
impl PartialEq<&str> for IsProprietaryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsProprietaryPropertyIri || *other == IS_PROPRIETARY_PROPERTY_LABEL
	}
}
impl PartialEq<IsProprietaryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsProprietaryPropertyIriOrLabel) -> bool {
		*self == IsProprietaryPropertyIri || *self == IS_PROPRIETARY_PROPERTY_LABEL
	}
}
