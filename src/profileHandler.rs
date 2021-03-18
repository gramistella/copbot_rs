use std::fs::OpenOptions;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Serialize,Deserialize,Debug)]
struct ProfileStorage{
    pub profiles: Vec<Profile>,
    pub cards: Vec<CreditCard>,
    pub gmail_accounts: Vec<Gmail>
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Profile{
    pub profile_name: String,
    pub full_name: String,
    pub email: String,
    pub tel: String,
    pub address: String,
    pub city: String,
    pub postcode: String,
    pub country: String,
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct CreditCard{
    pub profile_name: String,
    pub _type: String,
    pub number: String,
    pub month: String,
    pub year: String,
    pub cvv: String
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Gmail{
    pub profile_name: String,
    pub email: String,
    pub password: String
}


impl Default for Profile {
    fn default () -> Profile {
        Profile {
            profile_name: "".to_string(),
            full_name: "".to_string(),
            email: "".to_string(),
            tel: "".to_string(),
            address: "".to_string(),
            city: "".to_string(),
            postcode: "".to_string(),
            country: "".to_string()
        }
    }
}
impl Default for CreditCard {
    fn default () -> CreditCard {
        CreditCard {
            profile_name: "".to_string(),
            _type: "".to_string(),
            number: "".to_string(),
            month: "".to_string(),
            year: "".to_string(),
            cvv: "".to_string()
        }
    }
}
impl Default for Gmail {
    fn default () -> Gmail {
        Gmail {
            profile_name: "".to_string(),
            email: "".to_string(),
            password: "".to_string()
        }
    }
}
pub fn store_profile(user_profile: Profile) -> std::io::Result<()>{

    let mut profiles: Vec<Profile> = Vec::new();
    let mut cards: Vec<CreditCard> = Vec::new();
    let mut gmail_accounts: Vec<Gmail> = Vec::new();
    get_profiles(&mut profiles).ok();
    get_credit_cards(&mut cards).ok();
    get_gmail_accounts(&mut gmail_accounts).ok();
    let mut is_found: bool = false;
    for profile in profiles.iter_mut(){
        if profile.profile_name == user_profile.profile_name && user_profile.profile_name != "New"{

            profile.full_name = user_profile.full_name.clone();
            profile.email = user_profile.email.clone();
            profile.tel = user_profile.tel.clone();
            profile.address = user_profile.address.clone();
            profile.city = user_profile.city.clone();
            profile.postcode = user_profile.postcode.clone();
            profile.country = user_profile.country.clone();
            is_found = true;
            break;
        }
    }
    if !is_found && user_profile.profile_name != "New"{
        profiles.push(user_profile);
    }
    let storage = ProfileStorage{
        profiles,
        cards,
        gmail_accounts
    };
    let json = serde_json::to_string(&storage).unwrap();
    let mut file = OpenOptions::new().create(true).write(true).open("data.json").unwrap();
    file.set_len(0).ok();
    file.write_all(json.as_ref()).ok();
    Ok(())
}

pub fn store_credit(user_card: CreditCard) -> std::io::Result<()>{

    let mut profiles: Vec<Profile> = Vec::new();
    let mut cards: Vec<CreditCard> = Vec::new();
    let mut gmail_accounts: Vec<Gmail> = Vec::new();
    get_profiles(&mut profiles).ok();
    get_credit_cards(&mut cards).ok();
    get_gmail_accounts(&mut gmail_accounts).ok();
    let mut is_found: bool = false;
    for card in cards.iter_mut(){
        if card.profile_name == user_card.profile_name && user_card.profile_name != "New"{

            card._type = user_card._type.clone();
            card.number = user_card.number.clone();
            card.month = user_card.month.clone();
            card.year = user_card.year.clone();
            card.cvv = user_card.cvv.clone();
            is_found = true;
            break;
        }
    }
    if !is_found && user_card.profile_name != "New"{
        cards.push(user_card);
    }
    let storage = ProfileStorage{
        profiles,
        cards,
        gmail_accounts
    };

    let json = serde_json::to_string(&storage).unwrap();
    let mut file = OpenOptions::new().create(true).write(true).open("data.json").unwrap();
    file.set_len(0).ok();
    file.write_all(json.as_ref()).ok();
    Ok(())
}

pub fn store_gmail(user_account: Gmail) -> std::io::Result<()>{

    let mut profiles: Vec<Profile> = Vec::new();
    let mut cards: Vec<CreditCard> = Vec::new();
    let mut gmail_accounts: Vec<Gmail> = Vec::new();
    get_profiles(&mut profiles).ok();
    get_credit_cards(&mut cards).ok();
    get_gmail_accounts(&mut gmail_accounts).ok();
    let mut is_found: bool = false;
    for account in gmail_accounts.iter_mut(){
        if account.profile_name == user_account.profile_name && user_account.profile_name != "New"{

            account.email = user_account.email.clone();
            account.password = user_account.password.clone();
            is_found = true;
            break;
        }
    }
    if !is_found && user_account.profile_name != "New"{
        gmail_accounts.push(user_account);
    }
    let storage = ProfileStorage{
        profiles,
        cards,
        gmail_accounts
    };

    let json = serde_json::to_string(&storage).unwrap();
    let mut file = OpenOptions::new().create(true).write(true).open("data.json").unwrap();
    file.set_len(0).ok();
    file.write_all(json.as_ref()).ok();
    Ok(())
}

pub fn remove_profile(profile_name: String) -> std::io::Result<()>{
    let mut old_profiles: Vec<Profile> = Vec::new();
    let mut cards: Vec<CreditCard> = Vec::new();
    let mut gmail_accounts: Vec<Gmail> = Vec::new();
    get_profiles(&mut old_profiles).ok();
    get_credit_cards(&mut cards).ok();
    get_gmail_accounts(&mut gmail_accounts).ok();
    let mut profiles: Vec<Profile> = Vec::new();
    for profile in old_profiles.iter(){
        if profile.profile_name != profile_name {
            profiles.push(profile.clone());
        }
    }
    let storage = ProfileStorage{
        profiles,
        cards,
        gmail_accounts
    };

    let json = serde_json::to_string(&storage).unwrap();
    let mut file = OpenOptions::new().create(true).write(true).open("data.json").unwrap();
    file.set_len(0).ok();
    file.write_all(json.as_ref()).ok();
    Ok(())

}
pub fn remove_credit(profile_name: String) -> std::io::Result<()>{
    let mut profiles: Vec<Profile> = Vec::new();
    let mut old_cards: Vec<CreditCard> = Vec::new();
    let mut gmail_accounts: Vec<Gmail> = Vec::new();
    get_profiles(&mut profiles).ok();
    get_credit_cards(&mut old_cards).ok();
    get_gmail_accounts(&mut gmail_accounts).ok();
    let mut cards: Vec<CreditCard> = Vec::new();
    for profile in old_cards.iter(){
        if profile.profile_name != profile_name {
            cards.push(profile.clone());
        }
    }
    let storage = ProfileStorage{
        profiles,
        cards,
        gmail_accounts
    };
    let json = serde_json::to_string(&storage).unwrap();
    let mut file = OpenOptions::new().create(true).write(true).open("data.json").unwrap();
    file.set_len(0).ok();
    file.write_all(json.as_ref()).ok();
    Ok(())

}

pub fn remove_gmail(profile_name: String) -> std::io::Result<()>{
    let mut profiles: Vec<Profile> = Vec::new();
    let mut cards: Vec<CreditCard> = Vec::new();
    let mut old_gmail_accounts: Vec<Gmail> = Vec::new();
    get_profiles(&mut profiles).ok();
    get_credit_cards(&mut cards).ok();
    get_gmail_accounts(&mut old_gmail_accounts).ok();
    let mut gmail_accounts: Vec<Gmail> = Vec::new();
    for profile in old_gmail_accounts.iter(){
        if profile.profile_name != profile_name {
            gmail_accounts.push(profile.clone());
        }
    }
    let storage = ProfileStorage{
        profiles,
        cards,
        gmail_accounts
    };
    let json = serde_json::to_string(&storage).unwrap();
    let mut file = OpenOptions::new().create(true).write(true).open("data.json").unwrap();
    file.set_len(0).ok();
    file.write_all(json.as_ref()).ok();
    Ok(())

}

pub fn get_profile(profile_name: String, profile_found: &mut Profile) -> std::io::Result<()>{
    let mut json: serde_json::Value = serde_json::Value::default();
    get_json(&mut json).ok();
    for profiles in json["profiles"].as_array().iter(){
        for profile in profiles.iter() {
            if profile["profile_name"] == profile_name{
                let profile: Profile = serde_json::from_value(profile.clone()).unwrap();
                *profile_found = profile;
            }
        }
    }
    Ok(())
}

pub fn get_credit_card(profile_name: String, profile_found: &mut CreditCard) -> std::io::Result<()>{
    let mut json: serde_json::Value = serde_json::Value::default();
    get_json(&mut json).ok();
    for profiles in json["cards"].as_array().iter(){
        for profile in profiles.iter() {
            if profile["profile_name"] == profile_name{
                let profile: CreditCard = serde_json::from_value(profile.clone()).unwrap();
                *profile_found = profile;
            }
        }
    }
    Ok(())
}

pub fn get_gmail_account(profile_name: String, profile_found: &mut Gmail) -> std::io::Result<()>{
    let mut name_string: String = profile_name.clone();
    if profile_name.contains(".") {
        name_string.truncate(10);
    }
    println!("{}", name_string);
    let mut json: serde_json::Value = serde_json::Value::default();
    get_json(&mut json).ok();
    for profiles in json["gmail_accounts"].as_array().iter(){
        for profile in profiles.iter() {
            if profile["profile_name"].as_str().unwrap().contains(&name_string.clone()){
                let profile: Gmail = serde_json::from_value(profile.clone()).unwrap();
                *profile_found = profile;
            }
        }
    }
    Ok(())
}

pub fn get_profiles(profile_list: &mut Vec<Profile>) -> std::io::Result<()>{
    let mut json: serde_json::Value = serde_json::Value::default();
    get_json(&mut json).ok();
    for profiles in json["profiles"].as_array().iter(){
        for profile in profiles.iter() {
            let profile: Profile = serde_json::from_value(profile.to_owned()).unwrap();
            profile_list.push(profile);

        }
    }
    Ok(())
}

pub fn get_credit_cards(profile_list: &mut Vec<CreditCard>) -> std::io::Result<()>{
    let mut json: serde_json::Value = serde_json::Value::default();
    get_json(&mut json).ok();
    for profiles in json["cards"].as_array().iter(){
        for profile in profiles.iter() {
            let profile: CreditCard = serde_json::from_value(profile.to_owned()).unwrap();
            profile_list.push(profile);
        }
    }
    Ok(())
}

pub fn get_gmail_accounts(profile_list: &mut Vec<Gmail>) -> std::io::Result<()>{
    let mut json: serde_json::Value = serde_json::Value::default();
    get_json(&mut json).ok();
    for profiles in json["gmail_accounts"].as_array().iter(){
        for profile in profiles.iter() {
            let profile: Gmail = serde_json::from_value(profile.to_owned()).unwrap();
            profile_list.push(profile);
        }
    }
    Ok(())
}

fn get_json(json: &mut serde_json::Value) -> std::io::Result<()>{
    let file = OpenOptions::new().write(true).read(true).create(true).open("data.json").unwrap();
    let reader = BufReader::new(file);
    let _json_option = serde_json::from_reader(reader).ok();
    match _json_option {
        None => {
            set_up().ok();
        }
        Some(value) => {
            *json = value;
        }
    }
    Ok(())
}

fn set_up() -> std::io::Result<()> {
    let profiles: Vec<Profile> = Vec::new();
    let cards: Vec<CreditCard> = Vec::new();
    let gmail_accounts: Vec<Gmail> = Vec::new();
    let storage = ProfileStorage{
        profiles,
        cards,
        gmail_accounts
    };
    let json = serde_json::to_string(&storage).unwrap();
    let mut file = OpenOptions::new().create(true).write(true).open("data.json").unwrap();
    file.write_all(json.as_ref()).ok();
    Ok(())
}

pub fn write_to_debug(text: String) -> std::io::Result<()>{
    let mut file = OpenOptions::new().create(true).write(true).open("debug.html").unwrap();
    file.set_len(0).ok();
    file.write_all(text.as_ref()).ok();
    Ok(())
}