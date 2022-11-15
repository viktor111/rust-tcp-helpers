use find_port_to_bind::find_port_to_bind;
use socket_address_from_string_ip::socket_address_from_string_ip;
use validate_ip_address::validate_ip_address;
use create_listener::create_listener;
use listener_accept_conn::listener_accept_conn;
use read_message::read_message;
use write_message::write_message;
use check_size_of_msg::check_size_of_msg;

mod find_port_to_bind;
mod socket_address_from_string_ip;
mod validate_ip_address;
mod create_listener;
mod listener_accept_conn;
mod read_message;
mod write_message;
mod check_size_of_msg;

#[tokio::main]
async fn main() {
    let port_to_bind = find_port_to_bind(1, 5000).await.unwrap();

    if port_to_bind == 0 {
        println!("No port available");
    } else {
        println!("Port available: {}", port_to_bind);
    }

    let ip_address = "127.0.0.1:10";
    let is_valid_ip_address = validate_ip_address(ip_address.to_string()).unwrap();

    if is_valid_ip_address {
        println!("Valid IP address: {}", ip_address);
    } else {
        println!("Invalid IP address: {}", ip_address);
    }

    let socket_address = socket_address_from_string_ip("127.0.0.1:4001".to_string()).unwrap();
    println!("Socket address: {}", socket_address);

    let listener = create_listener(socket_address).await.unwrap();

    println!("Listener: {:?}", listener);

    loop{
        let conn = listener_accept_conn(&listener).await;

        if conn.is_ok() {
            let mut conn = conn.unwrap();
            check_size_of_msg(&mut conn.0).await.unwrap();
            let message = read_message(&mut conn.0).await.unwrap();
            println!("Message: {}", message);
            write_message(&mut conn.0, "Hello from server".to_string()).await.unwrap();
        }
    }
}
