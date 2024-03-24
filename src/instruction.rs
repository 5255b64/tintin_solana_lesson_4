use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum HelloWorldInstruction {
    // Define instruction here
    // For example:
    // Greeting,

    // 初始化记事本 初始化数据和owner
    Init { data: String, owner: String },

    // 读取记事本信息
    Read,

    // 写入记事本信息（完全覆盖）
    Write { data: String, owner: String },
}