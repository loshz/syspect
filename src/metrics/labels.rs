use prometheus_client::encoding::EncodeLabelSet;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ProcessLabels {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub pname: String,
}
