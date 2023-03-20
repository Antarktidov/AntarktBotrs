extern crate serenity;

//core::str;
use rand::Rng;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;

const TOKEN : &str = "Вставьте сюда ваш токен";

struct Handler;

impl EventHandler for Handler {

    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "?ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong?") {
                println!("Error giving message: {:?}", why)
            }
        }

        if msg.content.starts_with("!рандом ") {
            //let num: str = msg.content.slice_unchecked(0, 21);
            //let mut a : &str = "";
            unsafe {
                //let contentLen : &int = "msg.content".len();
                let num_str : &str  = msg.content.get_unchecked(14..msg.content.len());
                println!("Число-строка: {}", num_str);

                let num: i32 = num_str.parse::<i32>().unwrap();
                println!("Число: {}", num);

                let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
                let rnd_num: i32 = rng.gen_range(0..11);

                if rnd_num == num {
                    let response = "Верно, я загадал число ".to_owned() + &rnd_num.to_string();
                    if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                        println!("Error giving message: {:?}", why)
                    }
                }
                else {
                    let response = "А вот и нет, я загадал число ".to_owned() + &rnd_num.to_string();
                    if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                        println!("Error giving message: {:?}", why)
                    }
                }
            }
        }

        if msg.content.starts_with("!кнб "){
            unsafe {
                let user_item: &str = msg.content.get_unchecked(8..msg.content.len());
                println!("Предмет пользователя: {}", user_item);

                let bot_items_arr: [&str; 3] = ["камень", "ножницы", "бумага"];

                let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
                let rnd_index: i32 = rng.gen_range(0..3);

                let bot_item: &str = bot_items_arr[rnd_index as usize];

                if bot_item == user_item {
                    let response = "У меня ".to_owned() + &bot_item.to_string() + ", и у тебя " + &user_item.to_string() + ". Ничья";
                    if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                        println!("Error giving message: {:?}", why)
                    }
                }
                else if user_item == "камень" && bot_item == "ножницы" {
                    let response = "У меня ".to_owned() + &bot_item.to_string() + ", а у тебя " + &user_item.to_string() + ". Ты выйграл";
                    if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                        println!("Error giving message: {:?}", why)
                    }
                }
                else if user_item == "ножницы" && bot_item == "камень" {
                    let response = "У меня ".to_owned() + &bot_item.to_string() + ", а у тебя " + &user_item.to_string() + ". Я выйграл";
                    if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                        println!("Error giving message: {:?}", why)
                    }
                }
                else if user_item == "ножницы" && bot_item == "бумага" {
                    let response = "У меня ".to_owned() + &bot_item.to_string() + ", а у тебя " + &user_item.to_string() + ". Ты выйграл";
                    if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                        println!("Error giving message: {:?}", why)
                    }
                }
                else if user_item == "бумага" && bot_item == "ножницы" {
                    let response = "У меня ".to_owned() + &bot_item.to_string() + ", а у тебя " + &user_item.to_string() + ". Я выйграл";
                    if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                        println!("Error giving message: {:?}", why)
                    }
                }
                else if user_item == "камень" && bot_item == "бумага" {
                    let response = "У меня ".to_owned() + &bot_item.to_string() + ", а у тебя " + &user_item.to_string() + ". Я выйграл";
                    if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                        println!("Error giving message: {:?}", why)
                    }
                }
                else if user_item == "бумага" && bot_item == "камень" {
                    let response = "У меня ".to_owned() + &bot_item.to_string() + ", а у тебя " + &user_item.to_string() + ". Ты выйграл";
                    if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                        println!("Error giving message: {:?}", why)
                    }
                }
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}

fn main() {
    let mut client = Client::new(&TOKEN, Handler)
                    .expect("Error creating client");

    if let Err(msg) = client.start() {
        println!("Error: {:?}", msg);
    }
}
