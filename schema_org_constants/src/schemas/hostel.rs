/// <https://schema.org/Hostel>
pub const HOSTEL_IRI_HTTP: &str = "http://schema.org/Hostel";
/// <https://schema.org/Hostel>
pub const HOSTEL_IRI_HTTPS: &str = "https://schema.org/Hostel";
/// <https://schema.org/Hostel>
pub const HOSTEL_LABEL: &str = "Hostel";
pub struct HostelIri;
impl PartialEq<&str> for HostelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOSTEL_IRI_HTTP || *other == HOSTEL_IRI_HTTPS
	}
}
impl PartialEq<HostelIri> for &str {
	fn eq(&self, other: &HostelIri) -> bool {
		*self == HOSTEL_IRI_HTTP || *self == HOSTEL_IRI_HTTPS
	}
}
pub struct HostelIriOrLabel;
impl PartialEq<&str> for HostelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HostelIri || *other == HOSTEL_LABEL
	}
}
impl PartialEq<HostelIriOrLabel> for &str {
	fn eq(&self, other: &HostelIriOrLabel) -> bool {
		*self == HostelIri || *self == HOSTEL_LABEL
	}
}
