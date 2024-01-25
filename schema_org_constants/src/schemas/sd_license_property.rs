/// <https://schema.org/sdLicense>
pub const SD_LICENSE_PROPERTY_IRI_HTTP: &str = "http://schema.org/sdLicense";
/// <https://schema.org/sdLicense>
pub const SD_LICENSE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sdLicense";
/// <https://schema.org/sdLicense>
pub const SD_LICENSE_PROPERTY_LABEL: &str = "sdLicense";
pub struct SdLicensePropertyIri;
impl PartialEq<&str> for SdLicensePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SD_LICENSE_PROPERTY_IRI_HTTP || *other == SD_LICENSE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SdLicensePropertyIri> for &str {
	fn eq(&self, other: &SdLicensePropertyIri) -> bool {
		*self == SD_LICENSE_PROPERTY_IRI_HTTP || *self == SD_LICENSE_PROPERTY_IRI_HTTPS
	}
}
pub struct SdLicensePropertyIriOrLabel;
impl PartialEq<&str> for SdLicensePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SdLicensePropertyIri || *other == SD_LICENSE_PROPERTY_LABEL
	}
}
impl PartialEq<SdLicensePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SdLicensePropertyIriOrLabel) -> bool {
		*self == SdLicensePropertyIri || *self == SD_LICENSE_PROPERTY_LABEL
	}
}
