

use std::env;

use crate::api_error::ApiError;
use crate::db;
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};
use time::{Duration, OffsetDateTime};
use argon2::Config;
use rand::Rng;


#[derive(Serialize, Deserialize)]
pub struct UserAuthentication { //Structure recu dans le JSON authentification pour authentifier l utilisateur
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserInserable { //Structure inseree en BDD pour ajouter un user
    pub username : String,
    pub password: Vec<u8>,
}

#[derive( Queryable)]
pub struct User { //Structure recupere par requete BDD
    pub id: i32,
    pub username : String,
    pub password: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct UserEnvoye { //Structure a envoye dans la reponse http de l'admin
    pub id: i32,
    pub username: String,
}


#[derive(Serialize, Deserialize, PartialEq)]
pub struct Claims {  //Struture composant le JWT
    pub id: i32,
    pub username: String,
    #[serde(with = "jwt_numeric_date")]
    pub iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    pub exp: OffsetDateTime,
 }

 pub fn hash_password(password : String) -> Result<String, ApiError> { //Fct pour hash a String
        
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    let password = argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

    Ok(password)
}

pub fn chiffrer_password(password : String) -> Vec<u8> { //Fct pour chiffrer le password hashe


    let secret = env::var("KEY_BDD").expect("erreur chargement cle bdd");

    let nonce = env::var("NONCE").expect("erreur chargement du nonce");

    let key = Key::from_slice(secret.as_bytes()); // 32-bytes

    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(nonce.as_bytes()); // 96-bits 

    let ciphertext = cipher.encrypt(nonce, password.as_bytes().as_ref()).unwrap(); //chiffre le mot de passe qui est hashe

    ciphertext


}


 impl Claims {

    pub fn from_user(user : &UserEnvoye) -> Claims{  //Creation du JWT a partir des infos recuperees en BDD

        let iat1 = OffsetDateTime::now_utc();
        let exp1 = iat1 + Duration::hours(10);

        let iat = iat1
        .date()
        .with_hms_milli(iat1.hour(), iat1.minute(), iat1.second(), 0)
        .unwrap()
        .assume_utc();

        let exp = exp1
        .date()
        .with_hms_milli(exp1.hour(), exp1.minute(), exp1.second(), 0)
        .unwrap()
        .assume_utc();

        Claims {
            id : user.id,
            username : user.username.clone(),
            iat : iat,
            exp : exp,
        }
    }
     
 }


 impl UserInserable {

    pub fn from_user_recu(user : UserAuthentication) -> UserInserable { //Creation d un user inserable

        UserInserable {     //Renvoie la structure qui peut etre inseree en BDD
            username : user.username,
            password : chiffrer_password(user.password),
        }


    }
    
}

impl UserEnvoye {

    pub fn from_user(user : User) -> UserEnvoye{ //Fct pour renvoyer le nom et prenom dans le JSON lors de l'insertion de l'utilisateur

        UserEnvoye { 
            id: user.id, 
            username: user.username, 
        }
    }
    
}


impl User {


    pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> { //Verifier mot de passe de l'user qui veut se connecter  

        //On va dechiffrer le mot de passe de la BDD
        //On va comparer les hash entre le mot de passe BDD et celui envoyé a l'api

        let secret = env::var("KEY_BDD").expect("erreur chargement cle bdd");
        let nonce = env::var("NONCE").expect("erreur chargement du nonce");

        let key = Key::from_slice(secret.as_bytes()); // 32-bytes
        let cipher = ChaCha20Poly1305::new(key);

        let nonce = Nonce::from_slice(nonce.as_bytes()); // 96-bits; unique per message

        let password_bdd = cipher.decrypt(nonce, self.password.as_ref()).unwrap();  //Dechiffre le hash du mot de passe

        let password_bdd = String::from_utf8(password_bdd).expect("Echec lecture"); //Transforme le mot de passe hashe en String pour comparer

        argon2::verify_encoded(password_bdd.as_str(), password)   //Comparaison des hashs
            .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
    }


    pub fn find_by_username(username: String) -> Result<User, ApiError> { //Verifier que le username de l'user qui veut se connecter existe
        let mut conn = db::connection()?;

        //Fonction pour vérifier si le username existe bien

        let user = users::table
            .filter(users::username.eq(username))
            .first(&mut conn)?;


        Ok(user)
        
    }

    pub fn create(user: UserAuthentication) -> Result<UserEnvoye, ApiError> { //Fct pour créer un user à partir du JSON envoyé a l'api
        let mut conn = db::connection()?;

        //On va saler + hasher mot de passe
        //On recreer la variable en la passant en mutable pour ne
        //pas changer tout le code
        let mut user = user;

        user.password = hash_password(user.password)?;

        let user = UserInserable::from_user_recu(user);

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(& mut conn)?;

        let user = UserEnvoye::from_user(user);


    Ok(user)
    }

}





 mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }

}