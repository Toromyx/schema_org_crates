/// <https://schema.org/composer>
pub const COMPOSER_PROPERTY_IRI_HTTP: &str = "http://schema.org/composer";
/// <https://schema.org/composer>
pub const COMPOSER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/composer";
/// <https://schema.org/composer>
pub const COMPOSER_PROPERTY_LABEL: &str = "composer";
pub struct ComposerPropertyIri;
impl PartialEq<&str> for ComposerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPOSER_PROPERTY_IRI_HTTP || *other == COMPOSER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ComposerPropertyIri> for &str {
	fn eq(&self, other: &ComposerPropertyIri) -> bool {
		*self == COMPOSER_PROPERTY_IRI_HTTP || *self == COMPOSER_PROPERTY_IRI_HTTPS
	}
}
pub struct ComposerPropertyIriOrLabel;
impl PartialEq<&str> for ComposerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ComposerPropertyIri || *other == COMPOSER_PROPERTY_LABEL
	}
}
impl PartialEq<ComposerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ComposerPropertyIriOrLabel) -> bool {
		*self == ComposerPropertyIri || *self == COMPOSER_PROPERTY_LABEL
	}
}
