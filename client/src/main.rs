use std::io;

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // Greeting the user
    let mut name = greet();

    'main : loop {
        // Getting the command 
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Не удалось получить команду");
        
        // matching the command
        match input.trim() {
            "help" => print_help(),
            "exit" => break 'main,
            "start" => start_chat(name.clone().as_str()).await,
            "rename" => {name = rename()},
            "show" => show_channels().await,
            "create" => create_channel().await,
            _ => {println!("Unknown command: {}", input);}
        };

        // End of loop for new command
        println!("\nВведите команду: ");
    }
}




fn greet() -> String {
    println!("Добро пожаловать в консольный чат Term_chant!\n");
    println!("Введите ваше имя: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Не удалось ввести имя!");
    println!("Привет, {}!\n", name.trim());
    println!("Введите команду (или введите help для вызова справки): ");

    name
}


fn print_help() {
    println!("Список команд:");
    println!("help - вывод справки");
    println!("exit - выход из консоли");
    println!("start - начать чат");
    println!("rename - переименовать себя");
    println!("show - показать список каналов");
    println!("create - создать чат");
}

async fn start_chat(name: &str) {
    println!("Укажите название канала: ");
    let mut channel_name = String::new();
    io::stdin()
    .read_line(&mut channel_name)
    .expect("Не удалось ввести имя канала!");

    let body = StartData {
        channel_name: channel_name.trim().to_string(),
        name: name.trim().to_string()
    };

    let sse_client = reqwest::Client::new()
        .post("http://127.0.0.1:8080/api/v1/create/user")
        .json(&body)
        .send().await.unwrap();
    println!("{}", sse_client.status());
    loop {
        let mut message = String::new();
        io::stdin()
        .read_line(&mut message)
        .expect("Не получилось ввести сообщение!");
        if message.trim() == "!exit!" {
            break;
        }

        else { 
            let body = MessageData {
                message: message.trim().to_string(),
                user_name: name.trim().to_string(),
                channel_name: channel_name.trim().to_string()
            };
            println!("{:?}", body);
            let message = reqwest::Client::new();
            let response = message
                .post("http://127.0.0.1:8080/api/v1/create/message")
                .json(&body)
                .send()
                .await;
            match response {
                Ok(value) => {println!("{:?}", value)},
                Err(e) => {println!("Error: {}", e);},
            }
            // println!("{:?}", response);
            // println!("{:?}", response.status());
        }
    }
}

fn rename() -> String {
    let mut name = String::new();
    println!("Введите новое имя: ");
    io::stdin()
    .read_line(&mut name)
    .expect("Не удалось ввести имя!");
    println!("Привет, {}!\n", name.trim());
    name
}

async fn show_channels() {
    let body = reqwest::get("http://127.0.0.1:8080/api/v1/channels")
        .await.unwrap();
    let body: Vec<ChannelInfo> = serde_json::from_str(body.text().await.unwrap().as_str()).unwrap();
    println!("_____________________________________");
    for channel in body {
        println!("|[{}]", channel.channel_name);
        let mut index = 1;
        for user in channel.users {
            println!("|  {}) {}",index,  user.name);
            index += 1;
        }

    }
    println!("_____________________________________");

}

async fn create_channel() {
    println!("Укажите название канала: ");
    let mut channel_name = String::new();
    
    io::stdin()
    .read_line(&mut channel_name)
    .expect("Error reading");


    let body = {
        CreateChannelData{channel_name: channel_name.trim().to_string()}
    };

    let res = reqwest::Client::new()
        .post("http://127.0.0.1:8080/api/v1/create/channel")
        .json(&body)
        .send();
    println!("Response: {}", res.await.unwrap().status());
    println!("Канал создан.");

}


#[derive(Deserialize, Debug )]
pub struct ChannelInfo {
    pub channel_name: String,
    pub users: Vec<UserInfo>
}


#[derive(Deserialize, Debug )]
pub struct UserInfo {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateChannelData {
    channel_name: String,
}

#[derive(Serialize)]
pub struct StartData {
    channel_name: String,
    name: String,
}

#[derive(Serialize, Debug)]
pub struct MessageData {
    message: String,
    user_name: String,
    channel_name: String,
}