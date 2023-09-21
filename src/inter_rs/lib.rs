use ic_cdk::update;

mod declarations;
use declarations::counter_rs::counter_rs;

#[update]
async fn read() -> candid::Nat {
    counter_rs.read().await.unwrap().0
}

#[update]
async fn inc() {
    counter_rs.inc().await.unwrap()
}

#[update]
async fn write(input: candid::Nat) {
    counter_rs.write(input).await.unwrap()
}
