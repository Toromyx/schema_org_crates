/// <https://schema.org/LaserDiscFormat>
pub const LASER_DISC_FORMAT_IRI_HTTP: &str = "http://schema.org/LaserDiscFormat";
/// <https://schema.org/LaserDiscFormat>
pub const LASER_DISC_FORMAT_IRI_HTTPS: &str = "https://schema.org/LaserDiscFormat";
/// <https://schema.org/LaserDiscFormat>
pub const LASER_DISC_FORMAT_LABEL: &str = "LaserDiscFormat";
pub struct LaserDiscFormatIri;
impl PartialEq<&str> for LaserDiscFormatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LASER_DISC_FORMAT_IRI_HTTP || *other == LASER_DISC_FORMAT_IRI_HTTPS
	}
}
impl PartialEq<LaserDiscFormatIri> for &str {
	fn eq(&self, other: &LaserDiscFormatIri) -> bool {
		*self == LASER_DISC_FORMAT_IRI_HTTP || *self == LASER_DISC_FORMAT_IRI_HTTPS
	}
}
pub struct LaserDiscFormatIriOrLabel;
impl PartialEq<&str> for LaserDiscFormatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LaserDiscFormatIri || *other == LASER_DISC_FORMAT_LABEL
	}
}
impl PartialEq<LaserDiscFormatIriOrLabel> for &str {
	fn eq(&self, other: &LaserDiscFormatIriOrLabel) -> bool {
		*self == LaserDiscFormatIri || *self == LASER_DISC_FORMAT_LABEL
	}
}
