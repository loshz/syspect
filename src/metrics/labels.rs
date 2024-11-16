use prometheus_client::encoding::EncodeLabelSet;

use crate::bpf::ffi::Process;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ProcessLabels {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub pname: String,
}

impl From<Process> for ProcessLabels {
    fn from(process: Process) -> Self {
        Self {
            pid: process.pid as u32,
            pname: String::from_utf8_lossy(&process.pname)
                .trim_matches(char::from(0))
                .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bpf::ffi::Process;

    use super::ProcessLabels;

    #[test]
    fn test_processlabels_from_process() {
        let v: Vec<u8> = vec![
            115, 121, 115, 112, 101, 99, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 233, 233, 0, 0,
        ];
        let process = Process::from(v);
        let labels = ProcessLabels::from(process);
        assert_eq!("syspect", labels.pname);
        assert_eq!(59881, labels.pid);
    }
}
