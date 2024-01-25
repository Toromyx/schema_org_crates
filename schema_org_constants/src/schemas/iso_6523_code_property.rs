/// <https://schema.org/iso6523Code>
pub const ISO_6523_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/iso6523Code";
/// <https://schema.org/iso6523Code>
pub const ISO_6523_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/iso6523Code";
/// <https://schema.org/iso6523Code>
pub const ISO_6523_CODE_PROPERTY_LABEL: &str = "iso6523Code";
pub struct Iso6523CodePropertyIri;
impl PartialEq<&str> for Iso6523CodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ISO_6523_CODE_PROPERTY_IRI_HTTP || *other == ISO_6523_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<Iso6523CodePropertyIri> for &str {
	fn eq(&self, other: &Iso6523CodePropertyIri) -> bool {
		*self == ISO_6523_CODE_PROPERTY_IRI_HTTP || *self == ISO_6523_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct Iso6523CodePropertyIriOrLabel;
impl PartialEq<&str> for Iso6523CodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Iso6523CodePropertyIri || *other == ISO_6523_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<Iso6523CodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &Iso6523CodePropertyIriOrLabel) -> bool {
		*self == Iso6523CodePropertyIri || *self == ISO_6523_CODE_PROPERTY_LABEL
	}
}
