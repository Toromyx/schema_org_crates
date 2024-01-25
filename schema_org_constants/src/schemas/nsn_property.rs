/// <https://schema.org/nsn>
pub const NSN_PROPERTY_IRI_HTTP: &str = "http://schema.org/nsn";
/// <https://schema.org/nsn>
pub const NSN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/nsn";
/// <https://schema.org/nsn>
pub const NSN_PROPERTY_LABEL: &str = "nsn";
pub struct NsnPropertyIri;
impl PartialEq<&str> for NsnPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NSN_PROPERTY_IRI_HTTP || *other == NSN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NsnPropertyIri> for &str {
	fn eq(&self, other: &NsnPropertyIri) -> bool {
		*self == NSN_PROPERTY_IRI_HTTP || *self == NSN_PROPERTY_IRI_HTTPS
	}
}
pub struct NsnPropertyIriOrLabel;
impl PartialEq<&str> for NsnPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NsnPropertyIri || *other == NSN_PROPERTY_LABEL
	}
}
impl PartialEq<NsnPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NsnPropertyIriOrLabel) -> bool {
		*self == NsnPropertyIri || *self == NSN_PROPERTY_LABEL
	}
}
