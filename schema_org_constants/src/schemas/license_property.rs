/// <https://schema.org/license>
pub const LICENSE_PROPERTY_IRI_HTTP: &str = "http://schema.org/license";
/// <https://schema.org/license>
pub const LICENSE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/license";
/// <https://schema.org/license>
pub const LICENSE_PROPERTY_LABEL: &str = "license";
pub struct LicensePropertyIri;
impl PartialEq<&str> for LicensePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LICENSE_PROPERTY_IRI_HTTP || *other == LICENSE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LicensePropertyIri> for &str {
	fn eq(&self, other: &LicensePropertyIri) -> bool {
		*self == LICENSE_PROPERTY_IRI_HTTP || *self == LICENSE_PROPERTY_IRI_HTTPS
	}
}
pub struct LicensePropertyIriOrLabel;
impl PartialEq<&str> for LicensePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LicensePropertyIri || *other == LICENSE_PROPERTY_LABEL
	}
}
impl PartialEq<LicensePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LicensePropertyIriOrLabel) -> bool {
		*self == LicensePropertyIri || *self == LICENSE_PROPERTY_LABEL
	}
}
