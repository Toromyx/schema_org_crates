/// <https://schema.org/installUrl>
pub const INSTALL_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/installUrl";
/// <https://schema.org/installUrl>
pub const INSTALL_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/installUrl";
/// <https://schema.org/installUrl>
pub const INSTALL_URL_PROPERTY_LABEL: &str = "installUrl";
pub struct InstallUrlPropertyIri;
impl PartialEq<&str> for InstallUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INSTALL_URL_PROPERTY_IRI_HTTP || *other == INSTALL_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InstallUrlPropertyIri> for &str {
	fn eq(&self, other: &InstallUrlPropertyIri) -> bool {
		*self == INSTALL_URL_PROPERTY_IRI_HTTP || *self == INSTALL_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct InstallUrlPropertyIriOrLabel;
impl PartialEq<&str> for InstallUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InstallUrlPropertyIri || *other == INSTALL_URL_PROPERTY_LABEL
	}
}
impl PartialEq<InstallUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InstallUrlPropertyIriOrLabel) -> bool {
		*self == InstallUrlPropertyIri || *self == INSTALL_URL_PROPERTY_LABEL
	}
}
