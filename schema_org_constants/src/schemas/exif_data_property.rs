/// <https://schema.org/exifData>
pub const EXIF_DATA_PROPERTY_IRI_HTTP: &str = "http://schema.org/exifData";
/// <https://schema.org/exifData>
pub const EXIF_DATA_PROPERTY_IRI_HTTPS: &str = "https://schema.org/exifData";
/// <https://schema.org/exifData>
pub const EXIF_DATA_PROPERTY_LABEL: &str = "exifData";
pub struct ExifDataPropertyIri;
impl PartialEq<&str> for ExifDataPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXIF_DATA_PROPERTY_IRI_HTTP || *other == EXIF_DATA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExifDataPropertyIri> for &str {
	fn eq(&self, other: &ExifDataPropertyIri) -> bool {
		*self == EXIF_DATA_PROPERTY_IRI_HTTP || *self == EXIF_DATA_PROPERTY_IRI_HTTPS
	}
}
pub struct ExifDataPropertyIriOrLabel;
impl PartialEq<&str> for ExifDataPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExifDataPropertyIri || *other == EXIF_DATA_PROPERTY_LABEL
	}
}
impl PartialEq<ExifDataPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExifDataPropertyIriOrLabel) -> bool {
		*self == ExifDataPropertyIri || *self == EXIF_DATA_PROPERTY_LABEL
	}
}
