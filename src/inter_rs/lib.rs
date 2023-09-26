use ic_cdk::update;

mod declarations;
use candid::Principal;
use declarations::counter_rs::counter_rs;
use declarations::ledger::{ledger, Account, TransferFromArgs};

#[update]
async fn read() -> candid::Nat {
    counter_rs.read().await.unwrap().0
}

#[update]
async fn inc() {
    ledger
        .icrc_2_transfer_from(TransferFromArgs {
            to: Account {
                owner: Principal::from_slice(&[]),
                subaccount: None,
            },
            fee: None,
            spender_subaccount: None,
            from: Account {
                owner: Principal::from_slice(&[]),
                subaccount: None,
            },
            memo: None,
            created_at_time: None,
            amount: 5.into(),
        })
        .await
        .unwrap();
}

#[update]
async fn write(input: candid::Nat) {
    counter_rs.write(input).await.unwrap()
}
