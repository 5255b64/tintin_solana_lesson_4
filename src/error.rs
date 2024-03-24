use solana_program::program_error::ProgramError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomError
{
    // 文本内容超出长度限制
    // #[error("文本内容超出长度限制")]
    LengthLimitError,

    // 授权错误
    // #[error("授权错误：重复初始化")]
    AuthorizationErrorDoubleInit,

    // #[error("授权错误：缺少写入权限")]
    AuthorizationErrorNoWritePermission,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        match e {
            CustomError::LengthLimitError => ProgramError::Custom(1),
            CustomError::AuthorizationErrorDoubleInit => ProgramError::Custom(2),
            CustomError::AuthorizationErrorNoWritePermission => ProgramError::Custom(3)
        }
    }
}