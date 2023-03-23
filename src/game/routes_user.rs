use std::env;

use crate::game::*;
use crate::api_error::ApiError;

use actix_web::{ post, web,  HttpResponse, HttpRequest};
use jsonwebtoken::{ encode, decode, Header, EncodingKey, DecodingKey, Validation};


#[post("/login")]
pub async fn sign_in(credentials: web::Json<UserAuthentication>) -> Result<HttpResponse, ApiError> {

    let credentials = credentials.into_inner();

    //Verifie si le mail existe

    let user = User::find_by_username(credentials.username)
    .map_err(|e| {
        match e.status_code {
            404 => ApiError::new(401, "Credentials not valid!".to_string()),
            _ => e,
        }
    })?;

    //Verifie si le password est ok

    let is_valid = user.verify_password(credentials.password.as_bytes())?;


    if is_valid == true {

        let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

        let user = UserEnvoye::from_user(user); //Convertion vers la bonne structure

        let my_claims = Claims::from_user(&user); //Creation du corps du token

        let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref())).unwrap(); //Creation du jwt

        let tok = "Bearer ".to_string() + &token;

        Ok(HttpResponse::Ok()
            .insert_header(("Authorization", tok))
            .json(user)
        )
    }
    else {

        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }


}

#[post("/users")]
async fn create_user(user: web::Json<UserAuthentication>) -> Result<HttpResponse, ApiError> { //Enregistre un user

    let user = user.into_inner();

    let user = User::create(user)?;

    let _ = Score::create(user.username.clone())?;

    Ok(HttpResponse::Ok().json(user))

 
}

#[post("/snake")]
async fn update_score_snake(req : HttpRequest, score :  web::Json<ScoreJoueur>) -> Result<HttpResponse, ApiError>  {

    let header = req.headers().get("Authorization").unwrap();

    let headerhttp = header.to_str().unwrap();

    let jwt = headerhttp.split("Bearer ").collect::<Vec<&str>>()[1];

    let _claim = decode::<Claims>(jwt, &DecodingKey::from_secret("un big secret jwt".as_ref()), &Validation::default()).unwrap();

    let score = score.into_inner();

    let _ = Score::update_score(score.username.clone(), score.score)?;

    let score = Score::get_score(score.username)?;

    Ok(HttpResponse::Ok().json(score))



}

#[post("/snake/top")]

async fn get_top_snake() -> Result<HttpResponse, ApiError>  {

    let top = Score::get_score_top()?;

    Ok(HttpResponse::Ok().json(top))

}



pub fn routes_user(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_in);
    cfg.service(create_user);

}