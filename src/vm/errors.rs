// TODO: maybe also add VirtualMachineError with errors like LoadProgramError

#[derive(Debug)]
pub enum FdeError {
    UnknownInstruction { _nibbles: [u8; 4] },
    SubroutineReturn,
    NibblesFetch { _opcode_bytes: [u8; 2] },
    OpcodeFetch,
}

impl std::fmt::Display for FdeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FdeError {}
