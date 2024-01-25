/// <https://schema.org/acquireLicensePage>
pub const ACQUIRE_LICENSE_PAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/acquireLicensePage";
/// <https://schema.org/acquireLicensePage>
pub const ACQUIRE_LICENSE_PAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/acquireLicensePage";
/// <https://schema.org/acquireLicensePage>
pub const ACQUIRE_LICENSE_PAGE_PROPERTY_LABEL: &str = "acquireLicensePage";
pub struct AcquireLicensePagePropertyIri;
impl PartialEq<&str> for AcquireLicensePagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACQUIRE_LICENSE_PAGE_PROPERTY_IRI_HTTP
			|| *other == ACQUIRE_LICENSE_PAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AcquireLicensePagePropertyIri> for &str {
	fn eq(&self, other: &AcquireLicensePagePropertyIri) -> bool {
		*self == ACQUIRE_LICENSE_PAGE_PROPERTY_IRI_HTTP
			|| *self == ACQUIRE_LICENSE_PAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct AcquireLicensePagePropertyIriOrLabel;
impl PartialEq<&str> for AcquireLicensePagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AcquireLicensePagePropertyIri || *other == ACQUIRE_LICENSE_PAGE_PROPERTY_LABEL
	}
}
impl PartialEq<AcquireLicensePagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AcquireLicensePagePropertyIriOrLabel) -> bool {
		*self == AcquireLicensePagePropertyIri || *self == ACQUIRE_LICENSE_PAGE_PROPERTY_LABEL
	}
}
