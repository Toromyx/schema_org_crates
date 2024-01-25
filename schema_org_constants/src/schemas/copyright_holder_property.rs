/// <https://schema.org/copyrightHolder>
pub const COPYRIGHT_HOLDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/copyrightHolder";
/// <https://schema.org/copyrightHolder>
pub const COPYRIGHT_HOLDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/copyrightHolder";
/// <https://schema.org/copyrightHolder>
pub const COPYRIGHT_HOLDER_PROPERTY_LABEL: &str = "copyrightHolder";
pub struct CopyrightHolderPropertyIri;
impl PartialEq<&str> for CopyrightHolderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COPYRIGHT_HOLDER_PROPERTY_IRI_HTTP
			|| *other == COPYRIGHT_HOLDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CopyrightHolderPropertyIri> for &str {
	fn eq(&self, other: &CopyrightHolderPropertyIri) -> bool {
		*self == COPYRIGHT_HOLDER_PROPERTY_IRI_HTTP || *self == COPYRIGHT_HOLDER_PROPERTY_IRI_HTTPS
	}
}
pub struct CopyrightHolderPropertyIriOrLabel;
impl PartialEq<&str> for CopyrightHolderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CopyrightHolderPropertyIri || *other == COPYRIGHT_HOLDER_PROPERTY_LABEL
	}
}
impl PartialEq<CopyrightHolderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CopyrightHolderPropertyIriOrLabel) -> bool {
		*self == CopyrightHolderPropertyIri || *self == COPYRIGHT_HOLDER_PROPERTY_LABEL
	}
}
