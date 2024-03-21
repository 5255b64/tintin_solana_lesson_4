use solana_program::program_error::ProgramError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomError{
    IncorrectProgramId,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        match(e){
            CustomError::IncorrectProgramId => ProgramError::IncorrectProgramId
        }
    }
}