/// <https://schema.org/iupacName>
pub const IUPAC_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/iupacName";
/// <https://schema.org/iupacName>
pub const IUPAC_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/iupacName";
/// <https://schema.org/iupacName>
pub const IUPAC_NAME_PROPERTY_LABEL: &str = "iupacName";
pub struct IupacNamePropertyIri;
impl PartialEq<&str> for IupacNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IUPAC_NAME_PROPERTY_IRI_HTTP || *other == IUPAC_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IupacNamePropertyIri> for &str {
	fn eq(&self, other: &IupacNamePropertyIri) -> bool {
		*self == IUPAC_NAME_PROPERTY_IRI_HTTP || *self == IUPAC_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct IupacNamePropertyIriOrLabel;
impl PartialEq<&str> for IupacNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IupacNamePropertyIri || *other == IUPAC_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<IupacNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &IupacNamePropertyIriOrLabel) -> bool {
		*self == IupacNamePropertyIri || *self == IUPAC_NAME_PROPERTY_LABEL
	}
}
