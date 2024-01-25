/// <https://schema.org/DVDFormat>
pub const DVD_FORMAT_IRI_HTTP: &str = "http://schema.org/DVDFormat";
/// <https://schema.org/DVDFormat>
pub const DVD_FORMAT_IRI_HTTPS: &str = "https://schema.org/DVDFormat";
/// <https://schema.org/DVDFormat>
pub const DVD_FORMAT_LABEL: &str = "DVDFormat";
pub struct DvdFormatIri;
impl PartialEq<&str> for DvdFormatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DVD_FORMAT_IRI_HTTP || *other == DVD_FORMAT_IRI_HTTPS
	}
}
impl PartialEq<DvdFormatIri> for &str {
	fn eq(&self, other: &DvdFormatIri) -> bool {
		*self == DVD_FORMAT_IRI_HTTP || *self == DVD_FORMAT_IRI_HTTPS
	}
}
pub struct DvdFormatIriOrLabel;
impl PartialEq<&str> for DvdFormatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DvdFormatIri || *other == DVD_FORMAT_LABEL
	}
}
impl PartialEq<DvdFormatIriOrLabel> for &str {
	fn eq(&self, other: &DvdFormatIriOrLabel) -> bool {
		*self == DvdFormatIri || *self == DVD_FORMAT_LABEL
	}
}
