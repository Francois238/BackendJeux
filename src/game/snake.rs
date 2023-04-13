use crate::api_error::ApiError;
use crate::db;
use crate::schema::snake;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = snake)]
pub struct ScoreJoueur { //Structure inseree en BDD pour ajouter un user
    pub username : String,
    pub score: i32,
}


#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = snake)]
pub struct ScoreRecu { //Structure inseree en BDD pour modifier un score
    pub score: i32,
}

#[derive( Queryable)]
pub struct Score { //Structure recupere par requete BDD
    pub id: i32,
    pub username : String,
    pub score: i32,
}


impl Score{

    pub fn create(user: String) -> Result<(), ApiError> { 

        let mut connection = db::connection()?;

        let score = ScoreJoueur {
            username: user,
            score: 0
        };

        diesel::insert_into(snake::table)
            .values(score)
            .execute(&mut connection)?;


        Ok(())

    }

    pub fn update_score(user: String, score: i32) -> Result<usize, ApiError> { 

        let mut connection = db::connection()?;

        let old_score = snake::table
            .filter(snake::username.eq(user.clone()))
            .first::<Score>(&mut connection)?;

        if old_score.score < score { //Si le score est plus grand on update
            let score= diesel::update(snake::table.filter(snake::username.eq(user)))
            .set(snake::score.eq(score))
            .execute(&mut connection)?;

            Ok(score)
        }

        else{
            Ok(0)
        }

    }

    pub fn get_score(user: String) -> Result<ScoreJoueur, ApiError> { 

        let mut connection = db::connection()?;

        let score = snake::table
            .filter(snake::username.eq(user))
            .first::<Score>(&mut connection)?;

        Ok(ScoreJoueur {
            username: score.username,
            score: score.score
        })

    }


    pub fn get_score_top() -> Result<Vec<ScoreJoueur>, ApiError> { 

        let mut connection = db::connection()?;

        let score = snake::table
            .order(snake::score.desc())
            .limit(10)
            .load::<Score>(&mut connection)?;

        let mut score_envoye = Vec::new();

        for i in score {
            score_envoye.push(ScoreJoueur {
                username: i.username,
                score: i.score
            })
        }

        Ok(score_envoye)

    }

}
