pub fn get_address() -> String {
    use std::env;

    let args: Vec<String> = env::args().collect();
    let host: String;
    let port: String;

    if args.len() != 3 {
        host = String::from("127.0.0.1");
        port = String::from("8888");
    } else {
        host = args[1].to_string();
        port = args[2].to_string();
    }

    format!("{}:{}", host, port)
}
