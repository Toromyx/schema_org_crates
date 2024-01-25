/// <https://schema.org/InstallAction>
pub const INSTALL_ACTION_IRI_HTTP: &str = "http://schema.org/InstallAction";
/// <https://schema.org/InstallAction>
pub const INSTALL_ACTION_IRI_HTTPS: &str = "https://schema.org/InstallAction";
/// <https://schema.org/InstallAction>
pub const INSTALL_ACTION_LABEL: &str = "InstallAction";
pub struct InstallActionIri;
impl PartialEq<&str> for InstallActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INSTALL_ACTION_IRI_HTTP || *other == INSTALL_ACTION_IRI_HTTPS
	}
}
impl PartialEq<InstallActionIri> for &str {
	fn eq(&self, other: &InstallActionIri) -> bool {
		*self == INSTALL_ACTION_IRI_HTTP || *self == INSTALL_ACTION_IRI_HTTPS
	}
}
pub struct InstallActionIriOrLabel;
impl PartialEq<&str> for InstallActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InstallActionIri || *other == INSTALL_ACTION_LABEL
	}
}
impl PartialEq<InstallActionIriOrLabel> for &str {
	fn eq(&self, other: &InstallActionIriOrLabel) -> bool {
		*self == InstallActionIri || *self == INSTALL_ACTION_LABEL
	}
}
