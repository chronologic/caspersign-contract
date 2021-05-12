#[cfg(test)]
mod kv_storage;

#[cfg(test)]
mod tests {
    use super::kv_storage;
    use casper_types::{
        bytesrepr::{FromBytes, ToBytes}, CLTyped,
    };
    use kv_storage::KVstorageContract;
    
    fn generic_test<T: CLTyped + FromBytes + ToBytes>(
        fn_name: &str,
        key_name1: &str,
        key_name2: &str,
        value1: T,
        value2: T,
    ) -> (T, T) {
        let mut kv_storage = KVstorageContract::deploy();
        kv_storage.call_store_value(fn_name, key_name1, value1);
        kv_storage.call_store_value(fn_name, key_name2, value2);
        let check1: T = kv_storage.query_contract(key_name1).unwrap();
        let check2: T = kv_storage.query_contract(key_name2).unwrap();
        (check1, check2)
    }

    #[test]
    fn should_store_arbitrary_stringified_json_as_signature() {
        let signature1 = String::from(r#"{"id":1,"name":"Leanne Graham","username":"Bret","email":"Sincere@april.biz","address":{"street":"Kulas Light","suite":"Apt. 556","city":"Gwenborough","zipcode":"92998-3874","geo":{"lat":"-37.3159","lng":"81.1496"}},"phone":"1-770-736-8031 x56442","website":"hildegard.org","company":{"name":"Romaguera-Crona","catchPhrase":"Multi-layered client-server neural-net","bs":"harness real-time e-markets"}}"#);
        let signature2 = String::from(r#"{"id":2,"name":"Ervin Howell","username":"Antonette","email":"Shanna@melissa.tv","address":{"street":"Victor Plains","suite":"Suite 879","city":"Wisokyburgh","zipcode":"90566-7771","geo":{"lat":"-43.9509","lng":"-34.4618"}},"phone":"010-692-6593 x09125","website":"anastasia.net","company":{"name":"Deckow-Crist","catchPhrase":"Proactive didactic contingency","bs":"synergize scalable supply-chains"}}"#);
        let (ret1, ret2) = generic_test::<String>(
            "store_signature",
            "90ba8e42f0820b22e76e477624fb2aaa16b125caa144d94beca223324da54733",
            "ea8911548d4f74d1bd856c7d948c9c2f09b8ffd3c1b4c40035b7ddb3371c0fbb",
            signature1.clone(),
            signature2.clone(),
        );
        assert_eq!(signature1, ret1);
        assert_eq!(signature2, ret2);
    }

    #[test]
    fn should_revert_when_trying_to_overwrite_existing_key() {
        let signature1 = String::from("abc");
        let signature2 = String::from("def");
        let (ret1, ret2) = generic_test::<String>(
            "store_signature",
            "key1",
            "key1",
            signature1.clone(),
            signature2.clone(),
        );
        assert_eq!(signature1, ret1);
        assert_eq!(signature2, ret2);
    }
}
