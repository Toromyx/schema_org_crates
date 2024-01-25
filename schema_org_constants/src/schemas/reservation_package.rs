/// <https://schema.org/ReservationPackage>
pub const RESERVATION_PACKAGE_IRI_HTTP: &str = "http://schema.org/ReservationPackage";
/// <https://schema.org/ReservationPackage>
pub const RESERVATION_PACKAGE_IRI_HTTPS: &str = "https://schema.org/ReservationPackage";
/// <https://schema.org/ReservationPackage>
pub const RESERVATION_PACKAGE_LABEL: &str = "ReservationPackage";
pub struct ReservationPackageIri;
impl PartialEq<&str> for ReservationPackageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVATION_PACKAGE_IRI_HTTP || *other == RESERVATION_PACKAGE_IRI_HTTPS
	}
}
impl PartialEq<ReservationPackageIri> for &str {
	fn eq(&self, other: &ReservationPackageIri) -> bool {
		*self == RESERVATION_PACKAGE_IRI_HTTP || *self == RESERVATION_PACKAGE_IRI_HTTPS
	}
}
pub struct ReservationPackageIriOrLabel;
impl PartialEq<&str> for ReservationPackageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservationPackageIri || *other == RESERVATION_PACKAGE_LABEL
	}
}
impl PartialEq<ReservationPackageIriOrLabel> for &str {
	fn eq(&self, other: &ReservationPackageIriOrLabel) -> bool {
		*self == ReservationPackageIri || *self == RESERVATION_PACKAGE_LABEL
	}
}
