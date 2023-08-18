mod mta;

fn main() {
    let auth : mta::Auth = mta::Auth {
        username: "user".to_string(),
        password: "pass".to_string(),
    };
    let mta = mta::Mta::new(String::from("121.1.1.1"), 22003,auth, true);

    let res = mta.call("admin", "getServerVersion", vec!["test".to_string(), "teste2".to_string()]);
}
