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
        //по умолчанию
        let content: String = msg.content;
        let mut wiki_link: &str = "";
        let mut wiki_hosting: &str = "fandom";
        let mut lang: &str = "en";
        let mut is_file: bool = false;

        if content == "?ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong?") {
                println!("Error giving message: {:?}", why)
            }
        }

        if content.starts_with("!рандом ") {
            //let num: str =content.slice_unchecked(0, 21);
            //let mut a : &str = "";
            unsafe {
                //let contentLen : &int = "msg.content".len();
                let num_str : &str  =content.get_unchecked(14..content.len());
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

        if content.starts_with("!кнб "){
            unsafe {
                let user_item: &str = content.get_unchecked(8..content.len());
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
        if (content.contains("[[") &&  content.contains("]]")) || (content.contains("{{") &&  content.contains("}}")){
            println!("Обнаружена вики-ссылка!");
            if content.contains("[[") &&  content.contains("]]") {
                // Безопасный способ получить элемент строки по индексу, спасибо Бингу!
                wiki_link = match content.find("[") {
                    Some(start) => match content.find("]") {
                        Some(end) => &content[start + 2..end],
                        None => "", // Подстрока "]" не найдена
                    },
                    None => "", // Подстрока "[" не найдена
                };
                println!("Вики-ссылка: {}", wiki_link);
            }
            if content.contains("{{") &&  content.contains("}}") {
                // Безопасный способ получить элемент строки по индексу, спасибо Бингу!
                wiki_link = match content.find("{") {
                    Some(start) => match content.find("}") {
                        Some(end) => &content[start + 2..end],
                        None => "", // Подстрока "}" не найдена
                    },
                    None => "", // Подстрока "{" не найдена
                };
                println!("Вики-шаблон-ссылка: {}", wiki_link);
            }

            let wiki_link2: &String = &wiki_link.replace(" ", "_");
            wiki_link = wiki_link2;

            if wiki_link.starts_with("w:c:") {
                wiki_hosting = "fandom";

                unsafe {
                    wiki_link = wiki_link.get_unchecked(4..content.len());
                    println!("Вики-ссылка (2): {}", wiki_link);
                    /*let wiki_link2 = "?".to_owned() + &wiki_link.to_string();
                    wiki_link = &wiki_link2.to_string();*/

                    let s = "ru.test:Test";
                    let (new_s, old_s) = s.split_once(":").unwrap();
                    println!("new_s: {}", new_s); // ru.test
                    println!("old_s: {}", old_s); // Test

                    wiki_link = old_s;
                    println!("wiki_link: {}", wiki_link);

                    let (lang, url) = new_s.split_once(".").unwrap();

                    println!("lang: {}", lang);
                    println!("url: {}", url);

                    //println!("wiki_link2: {}", wiki_link2);

                    /*let l_url: &str = match wiki_link.find("?") {
                        Some(start) => match wiki_link.find(":") {
                            Some(end) => &wiki_link[start + 2..end],
                            None => "", // Подстрока ":" не найдена
                        },
                        None => "", // Подстрока "" не найдена
                    };
                    //let l_url = wiki_link[..]

                    println!("l_url: {}", l_url);*/
                }
            }

            if wiki_link.starts_with("ruwikipedia:") {
                wiki_hosting = "wikipedia";
                lang = "ru";
                unsafe {
                    wiki_link = wiki_link.get_unchecked(12..wiki_link.len());
                    println!("wiki_link: {}", wiki_link);
                }
            }

            if wiki_link.starts_with("commons:") {
                wiki_hosting = "commons";
                lang = "en";
                unsafe {
                    wiki_link = wiki_link.get_unchecked(8..wiki_link.len());
                    println!("wiki_link: {}", wiki_link);
                }
            }

            if wiki_link.starts_with("mw:") {
                wiki_hosting = "mediawiki";
                lang = "en";
                unsafe {
                    wiki_link = wiki_link.get_unchecked(3..wiki_link.len());
                    println!("wiki_link: {}", wiki_link);
                }
            }

            if wiki_link.starts_with("mh:") {
                wiki_hosting = "miraheze";
                lang = "en";
                unsafe {
                    wiki_link = wiki_link.get_unchecked(3..wiki_link.len());
                    println!("wiki_link: {}", wiki_link);

                    let (url, wiki_link) = wiki_link.split_once(":").unwrap();
                    println!("url: {}", url);
                    println!("wiki_link: {}", wiki_link); 
                }
            }

            if wiki_link.starts_with("File:") || wiki_link.starts_with("Файл:"){
                let (file, wiki_link) = wiki_link.split_once(":").unwrap();
                println!("file: {}", file);
                println!("wiki_link: {}", wiki_link);
                //wikiLink = $"Special:Redirect/file?wpvalue={wikiLink}";
                //wiki_link: String = "Special:Redirect/file?wpvalue=".to_owned() + wiki_link;

                let mut file_string = "Special:Redirect/file?wpvalue=".to_owned();
                file_string.push_str(wiki_link);
                println!("file_string: {}", file_string);
                //wiki_link = file_string

                is_file = true;
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
