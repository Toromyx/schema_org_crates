/// <https://schema.org/PublicHolidays>
pub const PUBLIC_HOLIDAYS_IRI_HTTP: &str = "http://schema.org/PublicHolidays";
/// <https://schema.org/PublicHolidays>
pub const PUBLIC_HOLIDAYS_IRI_HTTPS: &str = "https://schema.org/PublicHolidays";
/// <https://schema.org/PublicHolidays>
pub const PUBLIC_HOLIDAYS_LABEL: &str = "PublicHolidays";
pub struct PublicHolidaysIri;
impl PartialEq<&str> for PublicHolidaysIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLIC_HOLIDAYS_IRI_HTTP || *other == PUBLIC_HOLIDAYS_IRI_HTTPS
	}
}
impl PartialEq<PublicHolidaysIri> for &str {
	fn eq(&self, other: &PublicHolidaysIri) -> bool {
		*self == PUBLIC_HOLIDAYS_IRI_HTTP || *self == PUBLIC_HOLIDAYS_IRI_HTTPS
	}
}
pub struct PublicHolidaysIriOrLabel;
impl PartialEq<&str> for PublicHolidaysIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicHolidaysIri || *other == PUBLIC_HOLIDAYS_LABEL
	}
}
impl PartialEq<PublicHolidaysIriOrLabel> for &str {
	fn eq(&self, other: &PublicHolidaysIriOrLabel) -> bool {
		*self == PublicHolidaysIri || *self == PUBLIC_HOLIDAYS_LABEL
	}
}
