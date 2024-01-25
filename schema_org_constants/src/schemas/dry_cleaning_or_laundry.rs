/// <https://schema.org/DryCleaningOrLaundry>
pub const DRY_CLEANING_OR_LAUNDRY_IRI_HTTP: &str = "http://schema.org/DryCleaningOrLaundry";
/// <https://schema.org/DryCleaningOrLaundry>
pub const DRY_CLEANING_OR_LAUNDRY_IRI_HTTPS: &str = "https://schema.org/DryCleaningOrLaundry";
/// <https://schema.org/DryCleaningOrLaundry>
pub const DRY_CLEANING_OR_LAUNDRY_LABEL: &str = "DryCleaningOrLaundry";
pub struct DryCleaningOrLaundryIri;
impl PartialEq<&str> for DryCleaningOrLaundryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRY_CLEANING_OR_LAUNDRY_IRI_HTTP || *other == DRY_CLEANING_OR_LAUNDRY_IRI_HTTPS
	}
}
impl PartialEq<DryCleaningOrLaundryIri> for &str {
	fn eq(&self, other: &DryCleaningOrLaundryIri) -> bool {
		*self == DRY_CLEANING_OR_LAUNDRY_IRI_HTTP || *self == DRY_CLEANING_OR_LAUNDRY_IRI_HTTPS
	}
}
pub struct DryCleaningOrLaundryIriOrLabel;
impl PartialEq<&str> for DryCleaningOrLaundryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DryCleaningOrLaundryIri || *other == DRY_CLEANING_OR_LAUNDRY_LABEL
	}
}
impl PartialEq<DryCleaningOrLaundryIriOrLabel> for &str {
	fn eq(&self, other: &DryCleaningOrLaundryIriOrLabel) -> bool {
		*self == DryCleaningOrLaundryIri || *self == DRY_CLEANING_OR_LAUNDRY_LABEL
	}
}
