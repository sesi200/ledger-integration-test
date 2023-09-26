// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub enum MetadataValue {
    Int(candid::Int),
    Nat(candid::Nat),
    Blob(serde_bytes::ByteBuf),
    Text(String),
}

pub type Subaccount = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize)]
pub enum ChangeFeeCollector {
    SetTo(Account),
    Unset,
}

#[derive(CandidType, Deserialize)]
pub struct FeatureFlags {
    icrc2: bool,
}

#[derive(CandidType, Deserialize)]
pub struct UpgradeArgs {
    token_symbol: Option<String>,
    transfer_fee: Option<candid::Nat>,
    metadata: Option<Vec<(String, MetadataValue)>>,
    maximum_number_of_accounts: Option<u64>,
    accounts_overflow_trim_quantity: Option<u64>,
    change_fee_collector: Option<ChangeFeeCollector>,
    max_memo_length: Option<u16>,
    token_name: Option<String>,
    feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgsArchiveOptions {
    num_blocks_to_archive: u64,
    max_transactions_per_response: Option<u64>,
    trigger_threshold: u64,
    max_message_size_bytes: Option<u64>,
    cycles_for_archive_creation: Option<u64>,
    node_max_memory_size_bytes: Option<u64>,
    controller_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    decimals: Option<u8>,
    token_symbol: String,
    transfer_fee: candid::Nat,
    metadata: Vec<(String, MetadataValue)>,
    minting_account: Account,
    initial_balances: Vec<(Account, candid::Nat)>,
    maximum_number_of_accounts: Option<u64>,
    accounts_overflow_trim_quantity: Option<u64>,
    fee_collector_account: Option<Account>,
    archive_options: InitArgsArchiveOptions,
    max_memo_length: Option<u16>,
    token_name: String,
    feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType, Deserialize)]
pub enum LedgerArg {
    Upgrade(Option<UpgradeArgs>),
    Init(InitArgs),
}

pub type BlockIndex = candid::Nat;
#[derive(CandidType, Deserialize)]
pub struct GetBlocksArgs {
    start: BlockIndex,
    length: candid::Nat,
}

pub type Map = Vec<(String, Box<Value>)>;
#[derive(CandidType, Deserialize)]
pub enum Value {
    Int(candid::Int),
    Map(Map),
    Nat(candid::Nat),
    Nat64(u64),
    Blob(serde_bytes::ByteBuf),
    Text(String),
    Array(Vec<Box<Value>>),
}

pub type Block = Box<Value>;
#[derive(CandidType, Deserialize)]
pub struct BlockRange {
    blocks: Vec<Block>,
}

candid::define_function!(pub QueryBlockArchiveFn : (GetBlocksArgs) -> (
    BlockRange,
  ) query);
#[derive(CandidType, Deserialize)]
pub struct GetBlocksResponseArchivedBlocksItem {
    callback: QueryBlockArchiveFn,
    start: BlockIndex,
    length: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetBlocksResponse {
    certificate: Option<serde_bytes::ByteBuf>,
    first_index: BlockIndex,
    blocks: Vec<Block>,
    chain_length: u64,
    archived_blocks: Vec<GetBlocksResponseArchivedBlocksItem>,
}

#[derive(CandidType, Deserialize)]
pub struct DataCertificate {
    certificate: Option<serde_bytes::ByteBuf>,
    hash_tree: serde_bytes::ByteBuf,
}

pub type TxIndex = candid::Nat;
#[derive(CandidType, Deserialize)]
pub struct GetTransactionsRequest {
    start: TxIndex,
    length: candid::Nat,
}

pub type Timestamp = u64;
#[derive(CandidType, Deserialize)]
pub struct Burn {
    from: Account,
    memo: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<Timestamp>,
    amount: candid::Nat,
    spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Mint {
    to: Account,
    memo: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<Timestamp>,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Approve {
    fee: Option<candid::Nat>,
    from: Account,
    memo: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<Timestamp>,
    amount: candid::Nat,
    expected_allowance: Option<candid::Nat>,
    expires_at: Option<Timestamp>,
    spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Transfer {
    to: Account,
    fee: Option<candid::Nat>,
    from: Account,
    memo: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<Timestamp>,
    amount: candid::Nat,
    spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
    burn: Option<Burn>,
    kind: String,
    mint: Option<Mint>,
    approve: Option<Approve>,
    timestamp: Timestamp,
    transfer: Option<Transfer>,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionRange {
    transactions: Vec<Transaction>,
}

candid::define_function!(pub QueryArchiveFn : (GetTransactionsRequest) -> (
    TransactionRange,
  ) query);
#[derive(CandidType, Deserialize)]
pub struct GetTransactionsResponseArchivedTransactionsItem {
    callback: QueryArchiveFn,
    start: TxIndex,
    length: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsResponse {
    first_index: TxIndex,
    log_length: candid::Nat,
    transactions: Vec<Transaction>,
    archived_transactions: Vec<GetTransactionsResponseArchivedTransactionsItem>,
}

pub type Tokens = candid::Nat;
#[derive(CandidType, Deserialize)]
pub struct StandardRecord {
    url: String,
    name: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
    to: Account,
    fee: Option<Tokens>,
    memo: Option<serde_bytes::ByteBuf>,
    from_subaccount: Option<Subaccount>,
    created_at_time: Option<Timestamp>,
    amount: Tokens,
}

#[derive(CandidType, Deserialize)]
pub enum TransferError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    BadBurn {
        min_burn_amount: Tokens,
    },
    Duplicate {
        duplicate_of: BlockIndex,
    },
    BadFee {
        expected_fee: Tokens,
    },
    CreatedInFuture {
        ledger_time: Timestamp,
    },
    TooOld,
    InsufficientFunds {
        balance: Tokens,
    },
}

#[derive(CandidType, Deserialize)]
pub enum TransferResult {
    Ok(BlockIndex),
    Err(TransferError),
}

#[derive(CandidType, Deserialize)]
pub struct AllowanceArgs {
    account: Account,
    spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Allowance {
    allowance: candid::Nat,
    expires_at: Option<Timestamp>,
}

#[derive(CandidType, Deserialize)]
pub struct ApproveArgs {
    fee: Option<candid::Nat>,
    memo: Option<serde_bytes::ByteBuf>,
    from_subaccount: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<Timestamp>,
    amount: candid::Nat,
    expected_allowance: Option<candid::Nat>,
    expires_at: Option<Timestamp>,
    spender: Account,
}

#[derive(CandidType, Deserialize)]
pub enum ApproveError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    Duplicate {
        duplicate_of: BlockIndex,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    AllowanceChanged {
        current_allowance: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: Timestamp,
    },
    TooOld,
    Expired {
        ledger_time: Timestamp,
    },
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum ApproveResult {
    Ok(BlockIndex),
    Err(ApproveError),
}

#[derive(CandidType, Deserialize)]
pub struct TransferFromArgs {
    pub to: Account,
    pub fee: Option<Tokens>,
    pub spender_subaccount: Option<Subaccount>,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<Timestamp>,
    pub amount: Tokens,
}

#[derive(CandidType, Deserialize)]
pub enum TransferFromError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    InsufficientAllowance {
        allowance: Tokens,
    },
    BadBurn {
        min_burn_amount: Tokens,
    },
    Duplicate {
        duplicate_of: BlockIndex,
    },
    BadFee {
        expected_fee: Tokens,
    },
    CreatedInFuture {
        ledger_time: Timestamp,
    },
    TooOld,
    InsufficientFunds {
        balance: Tokens,
    },
}

#[derive(CandidType, Deserialize)]
pub enum TransferFromResult {
    Ok(BlockIndex),
    Err(TransferFromError),
}

pub struct Ledger(pub Principal);
impl Ledger {
    pub async fn get_blocks(&self, arg0: GetBlocksArgs) -> Result<(GetBlocksResponse,)> {
        ic_cdk::call(self.0, "get_blocks", (arg0,)).await
    }
    pub async fn get_data_certificate(&self) -> Result<(DataCertificate,)> {
        ic_cdk::call(self.0, "get_data_certificate", ()).await
    }
    pub async fn get_transactions(
        &self,
        arg0: GetTransactionsRequest,
    ) -> Result<(GetTransactionsResponse,)> {
        ic_cdk::call(self.0, "get_transactions", (arg0,)).await
    }
    pub async fn icrc_1_balance_of(&self, arg0: Account) -> Result<(Tokens,)> {
        ic_cdk::call(self.0, "icrc1_balance_of", (arg0,)).await
    }
    pub async fn icrc_1_decimals(&self) -> Result<(u8,)> {
        ic_cdk::call(self.0, "icrc1_decimals", ()).await
    }
    pub async fn icrc_1_fee(&self) -> Result<(Tokens,)> {
        ic_cdk::call(self.0, "icrc1_fee", ()).await
    }
    pub async fn icrc_1_metadata(&self) -> Result<(Vec<(String, MetadataValue)>,)> {
        ic_cdk::call(self.0, "icrc1_metadata", ()).await
    }
    pub async fn icrc_1_minting_account(&self) -> Result<(Option<Account>,)> {
        ic_cdk::call(self.0, "icrc1_minting_account", ()).await
    }
    pub async fn icrc_1_name(&self) -> Result<(String,)> {
        ic_cdk::call(self.0, "icrc1_name", ()).await
    }
    pub async fn icrc_1_supported_standards(&self) -> Result<(Vec<StandardRecord>,)> {
        ic_cdk::call(self.0, "icrc1_supported_standards", ()).await
    }
    pub async fn icrc_1_symbol(&self) -> Result<(String,)> {
        ic_cdk::call(self.0, "icrc1_symbol", ()).await
    }
    pub async fn icrc_1_total_supply(&self) -> Result<(Tokens,)> {
        ic_cdk::call(self.0, "icrc1_total_supply", ()).await
    }
    pub async fn icrc_1_transfer(&self, arg0: TransferArg) -> Result<(TransferResult,)> {
        ic_cdk::call(self.0, "icrc1_transfer", (arg0,)).await
    }
    pub async fn icrc_2_allowance(&self, arg0: AllowanceArgs) -> Result<(Allowance,)> {
        ic_cdk::call(self.0, "icrc2_allowance", (arg0,)).await
    }
    pub async fn icrc_2_approve(&self, arg0: ApproveArgs) -> Result<(ApproveResult,)> {
        ic_cdk::call(self.0, "icrc2_approve", (arg0,)).await
    }
    pub async fn icrc_2_transfer_from(
        &self,
        arg0: TransferFromArgs,
    ) -> Result<(TransferFromResult,)> {
        ic_cdk::call(self.0, "icrc2_transfer_from", (arg0,)).await
    }
}
pub const CANISTER_ID: Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 12, 1, 1]); // ajuq4-ruaaa-aaaaa-qaaga-cai
pub const ledger: Ledger = Ledger(CANISTER_ID);
