/// <https://schema.org/SoftwareApplication>
pub const SOFTWARE_APPLICATION_IRI_HTTP: &str = "http://schema.org/SoftwareApplication";
/// <https://schema.org/SoftwareApplication>
pub const SOFTWARE_APPLICATION_IRI_HTTPS: &str = "https://schema.org/SoftwareApplication";
/// <https://schema.org/SoftwareApplication>
pub const SOFTWARE_APPLICATION_LABEL: &str = "SoftwareApplication";
pub struct SoftwareApplicationIri;
impl PartialEq<&str> for SoftwareApplicationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOFTWARE_APPLICATION_IRI_HTTP || *other == SOFTWARE_APPLICATION_IRI_HTTPS
	}
}
impl PartialEq<SoftwareApplicationIri> for &str {
	fn eq(&self, other: &SoftwareApplicationIri) -> bool {
		*self == SOFTWARE_APPLICATION_IRI_HTTP || *self == SOFTWARE_APPLICATION_IRI_HTTPS
	}
}
pub struct SoftwareApplicationIriOrLabel;
impl PartialEq<&str> for SoftwareApplicationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SoftwareApplicationIri || *other == SOFTWARE_APPLICATION_LABEL
	}
}
impl PartialEq<SoftwareApplicationIriOrLabel> for &str {
	fn eq(&self, other: &SoftwareApplicationIriOrLabel) -> bool {
		*self == SoftwareApplicationIri || *self == SOFTWARE_APPLICATION_LABEL
	}
}
