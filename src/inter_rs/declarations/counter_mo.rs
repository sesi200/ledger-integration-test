// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

pub struct CounterMo(pub Principal);
impl CounterMo {
  pub async fn inc(&self) -> Result<()> {
    ic_cdk::call(self.0, "inc", ()).await
  }
  pub async fn read(&self) -> Result<(candid::Nat,)> {
    ic_cdk::call(self.0, "read", ()).await
  }
  pub async fn write(&self, arg0: candid::Nat) -> Result<()> {
    ic_cdk::call(self.0, "write", (arg0,)).await
  }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 2, 1, 1]); // bd3sg-teaaa-aaaaa-qaaba-cai
pub const counter_mo : CounterMo = CounterMo(CANISTER_ID);