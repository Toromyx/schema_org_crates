/// <https://schema.org/logo>
pub const LOGO_PROPERTY_IRI_HTTP: &str = "http://schema.org/logo";
/// <https://schema.org/logo>
pub const LOGO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/logo";
/// <https://schema.org/logo>
pub const LOGO_PROPERTY_LABEL: &str = "logo";
pub struct LogoPropertyIri;
impl PartialEq<&str> for LogoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOGO_PROPERTY_IRI_HTTP || *other == LOGO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LogoPropertyIri> for &str {
	fn eq(&self, other: &LogoPropertyIri) -> bool {
		*self == LOGO_PROPERTY_IRI_HTTP || *self == LOGO_PROPERTY_IRI_HTTPS
	}
}
pub struct LogoPropertyIriOrLabel;
impl PartialEq<&str> for LogoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LogoPropertyIri || *other == LOGO_PROPERTY_LABEL
	}
}
impl PartialEq<LogoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LogoPropertyIriOrLabel) -> bool {
		*self == LogoPropertyIri || *self == LOGO_PROPERTY_LABEL
	}
}
