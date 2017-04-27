use serde_json;

#[derive(Clone,Deserialize,Eq,Hash,PartialEq,Serialize)]
pub struct Client {
    address: String
}

#[test]
fn it_should_add_new_clients() {
    let mut clients = vec!();
    static TEST_STRING : &'static str = "test:1234";
    handle_client(&mut clients, &TEST_STRING.to_string());

    assert!(clients.len() == 1);
    for elem in clients {
        assert!(elem == Client{address:TEST_STRING.to_string()});
    }
}

#[test]
fn it_should_recognize_clients() {
    let mut clients = vec!();
    static TEST_STRING : &'static str = "test:1234";
    let first_result = handle_client(&mut clients, &TEST_STRING.to_string());
    let second_result = handle_client(&mut clients, &TEST_STRING.to_string());

    assert!(clients.len() == 1);
    assert!(first_result["clients"].as_array().unwrap().len() == 1);
    assert!(second_result == first_result);
}

#[test]
fn it_should_generate_response_for_new_client() {
    let mut clients = vec!();
    static TEST_STRING : &'static str = "test:1234";
    static SECOND_TEST_STRING : &'static str = "test2:5678";
    clients.push(Client { address:TEST_STRING.to_string() });
    let result = handle_client(&mut clients, &SECOND_TEST_STRING.to_string());

    assert!(result == json!({
        "clients": [
            { "address" : TEST_STRING.to_string() },
            { "address" : SECOND_TEST_STRING.to_string() }
        ]
    }));
}

pub fn handle_client(clients: &mut Vec<Client>, stream_name: &String) -> serde_json::Value {
    let new_client = Client { address: stream_name.to_string() };
    if ! clients.contains(&new_client) {
        clients.push(new_client);
    }
    let json_vector = clients.clone();
    return json!({ "clients" : json_vector });
}
