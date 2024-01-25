/// <https://schema.org/ImagingTest>
pub const IMAGING_TEST_IRI_HTTP: &str = "http://schema.org/ImagingTest";
/// <https://schema.org/ImagingTest>
pub const IMAGING_TEST_IRI_HTTPS: &str = "https://schema.org/ImagingTest";
/// <https://schema.org/ImagingTest>
pub const IMAGING_TEST_LABEL: &str = "ImagingTest";
pub struct ImagingTestIri;
impl PartialEq<&str> for ImagingTestIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IMAGING_TEST_IRI_HTTP || *other == IMAGING_TEST_IRI_HTTPS
	}
}
impl PartialEq<ImagingTestIri> for &str {
	fn eq(&self, other: &ImagingTestIri) -> bool {
		*self == IMAGING_TEST_IRI_HTTP || *self == IMAGING_TEST_IRI_HTTPS
	}
}
pub struct ImagingTestIriOrLabel;
impl PartialEq<&str> for ImagingTestIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ImagingTestIri || *other == IMAGING_TEST_LABEL
	}
}
impl PartialEq<ImagingTestIriOrLabel> for &str {
	fn eq(&self, other: &ImagingTestIriOrLabel) -> bool {
		*self == ImagingTestIri || *self == IMAGING_TEST_LABEL
	}
}
