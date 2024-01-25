/// <https://schema.org/faxNumber>
pub const FAX_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/faxNumber";
/// <https://schema.org/faxNumber>
pub const FAX_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/faxNumber";
/// <https://schema.org/faxNumber>
pub const FAX_NUMBER_PROPERTY_LABEL: &str = "faxNumber";
pub struct FaxNumberPropertyIri;
impl PartialEq<&str> for FaxNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FAX_NUMBER_PROPERTY_IRI_HTTP || *other == FAX_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FaxNumberPropertyIri> for &str {
	fn eq(&self, other: &FaxNumberPropertyIri) -> bool {
		*self == FAX_NUMBER_PROPERTY_IRI_HTTP || *self == FAX_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct FaxNumberPropertyIriOrLabel;
impl PartialEq<&str> for FaxNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FaxNumberPropertyIri || *other == FAX_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<FaxNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FaxNumberPropertyIriOrLabel) -> bool {
		*self == FaxNumberPropertyIri || *self == FAX_NUMBER_PROPERTY_LABEL
	}
}
