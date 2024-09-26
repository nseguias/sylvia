use sylvia::cw_multi_test::IntoAddr;
use sylvia::multitest::App;

use crate::{
    contract::sv::mt::{CodeId, CounterContractProxy},
    error::ContractError,
};

#[test]
fn instantiate() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);
    let owner = "owner".into_addr();

    // instantiate contract with count 42
    let contract = code_id.instantiate(42).call(&owner).unwrap();
    let count = contract.count().unwrap().count;
    assert_eq!(count, 42);

    // increment count from 42 to 43
    let _res = contract.increment_count().call(&owner).unwrap();
    let new_count = contract.count().unwrap().count;
    assert_eq!(new_count, 43);

    // set count to 0
    let _res = contract.set_count(0).call(&owner).unwrap();
    let reset_count = contract.count().unwrap().count;
    assert_eq!(reset_count, 0);

    // decrement below 0
    let res = contract.decrement_count().call(&owner);
    assert_eq!(res.unwrap_err(), ContractError::CannotDecrementCount {});
}
