use candid::{Principal, CandidType};
use serde::{Deserialize, Serialize};
use junobuild_utils::encode_doc_data;
use std::cell::RefCell;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct RequestParam{
    pub principal: String,
    pub product_name: String,
    pub price: i32,
    pub currency: String
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct HookContext {
    pub caller: Principal,
    pub data: DocContext,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct DocContext {
    pub collection: String,
    pub key: String,
    pub data: DocUpsert,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct DocUpsert {
    pub before: Option<Doc>,
    pub after: Doc,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Doc {
    pub owner: Principal,
    pub data: Vec<u8>,
    pub description: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub version: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct Note {
    pub name: String,
    pub price_cents: i32,
    pub currency: String
}

#[ic_cdk::update(name="Insert_Receipt")]
async fn insert_receipt(param: RequestParam) -> (bool, String) {
    let record = Note{
        name: param.product_name,
        price_cents: param.price,
        currency: param.currency
    };
    let encoded_data = encode_doc_data(&record).unwrap();
    let doc = Doc{
        owner: Principal::from_text(param.principal.clone()).unwrap(),
        data: encoded_data,
        description: Some("from test canister".to_string()),
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
        version: Some(1)
    };
    let doc_upsert = DocUpsert{
        before: None,
        after: doc
    };
    let doc_context = DocContext{
        collection: "notes".to_string(),
        key: ic_cdk::api::time().to_string(),
        data: doc_upsert
    };
    let hook_context = HookContext{
        caller: Principal::from_text(param.principal).unwrap(),
        data: doc_context
    };
    let get_result = ic_cdk::call::<(HookContext,), (bool, String, )>(Principal::from_text("vbfuz-pqaaa-aaaal-ajhaa-cai").unwrap(), "set_doc_from_canister", (hook_context,)).await;
    match get_result {
        Ok((result, error,)) => {
            (result, error)
        },
        Err((_, err)) => {
            (false, err.to_string())
        }
    }     
}
