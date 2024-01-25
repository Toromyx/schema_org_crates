/// <https://schema.org/BusStop>
pub const BUS_STOP_IRI_HTTP: &str = "http://schema.org/BusStop";
/// <https://schema.org/BusStop>
pub const BUS_STOP_IRI_HTTPS: &str = "https://schema.org/BusStop";
/// <https://schema.org/BusStop>
pub const BUS_STOP_LABEL: &str = "BusStop";
pub struct BusStopIri;
impl PartialEq<&str> for BusStopIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUS_STOP_IRI_HTTP || *other == BUS_STOP_IRI_HTTPS
	}
}
impl PartialEq<BusStopIri> for &str {
	fn eq(&self, other: &BusStopIri) -> bool {
		*self == BUS_STOP_IRI_HTTP || *self == BUS_STOP_IRI_HTTPS
	}
}
pub struct BusStopIriOrLabel;
impl PartialEq<&str> for BusStopIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusStopIri || *other == BUS_STOP_LABEL
	}
}
impl PartialEq<BusStopIriOrLabel> for &str {
	fn eq(&self, other: &BusStopIriOrLabel) -> bool {
		*self == BusStopIri || *self == BUS_STOP_LABEL
	}
}
