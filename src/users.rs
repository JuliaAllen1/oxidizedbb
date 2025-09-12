use argon2::password_hash::rand_core::OsRng;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use bbcode::BBCode;
use chrono::{Local, NaiveDate, NaiveDateTime};
use email_address::EmailAddress;
use rand::prelude::*;
#[allow(dead_code)]
pub struct User<'a> {
    username: &'a str,
    email: EmailAddress,
    nuid: u64,
    passsalt: SaltString,
    passhash: PasswordHash<'a>,
    last_logged_in: Option<chrono::NaiveDateTime>,
    created: chrono::NaiveDate,
    last_posted: Option<chrono::NaiveDateTime>,
    preferences: UserPreferences,
    profile: UserProfile,
}

pub struct UserPreferences {
    show_smilies: bool,
    show_signatures: bool,
    show_pfp: bool,
    show_img: bool,
}

pub struct UserProfile {
    real_name: String,
    location: String,
    website: String,
    aim: String,
    icq: String,
    //todo: pfp
    signature: String, // to be interpreted as bbcode
}

impl User<'_> {
    fn new(username: String, password: &str, email: EmailAddress) -> Self {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(OsRng);
        User {
            username: &username,
            email: email,
            nuid: rand::rng().next_u64(),
            passsalt: salt,
            passhash: argon2
                .hash_password(password, Self.passsalt)
                .expect("invalid user?"), // todo better handling
            last_logged_in: None,
            created: Local::now(),
            last_posted: None,
            preferences: UserPreferences {
                show_smilies: true,
                show_signatures: true,
                show_pfp: true,
                show_img: true,
            },
            profile: UserProfile {
                real_name: "",
                location: "",
                website: "",
                aim: "",
                icq: "",
                signature: "",
            },
        }
    }
}
