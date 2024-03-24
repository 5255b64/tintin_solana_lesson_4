use borsh::{BorshDeserialize, BorshSerialize};

/// Define the type of state stored in accounts
/// 记事本数据结构
#[derive(BorshSerialize, BorshDeserialize, Debug, Eq, PartialEq, Clone)]
pub struct Notebook {
    pub data:String, // 记事本内容
    pub owner:String, // 拥有记事本修改权限的成员（内容格式为pubkey）
    pub is_init:bool, // 是否已进行初始化
}

pub const CONTEXT_LENGTH_LIMIT: u32 = 100; // 记事本内容长度限制，单位字节