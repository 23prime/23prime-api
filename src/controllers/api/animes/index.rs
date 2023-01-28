use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::types::animes::{StrictAnime, StrictAnimes};
use entity::anime::{Column as AnimeColumn, Entity as AnimeEntity};

#[derive(Debug, Deserialize, Serialize)]
struct ResponseBody {
    animes: StrictAnimes,
}

#[derive(Debug, Deserialize, Serialize)]
struct ErrorResponseBody {
    reason: String,
    successful_animes: StrictAnimes,
    failed_anime: StrictAnime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathParams {
    year: i32,
    season: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BodyParams {
    animes: StrictAnimes,
}

type AppData = web::Data<AppState>;

pub async fn get(data: AppData) -> impl Responder {
    let found_animes = AnimeEntity::find().all(&data.db).await;

    if found_animes.is_err() {
        error!("Failed to find animes from DB.");
        return HttpResponse::InternalServerError().finish();
    }

    let mut animes = StrictAnime::new_by_models(found_animes.unwrap());
    animes.sort();
    return HttpResponse::Ok().json(ResponseBody { animes });
}

pub async fn get_by_year(data: AppData, path_params: web::Path<PathParams>) -> impl Responder {
    let found_animes = AnimeEntity::find()
        .filter(AnimeColumn::Year.eq(path_params.year))
        .all(&data.db)
        .await;

    if found_animes.is_err() {
        error!("Failed to find animes from DB.");
        return HttpResponse::InternalServerError().finish();
    }

    let mut animes = StrictAnime::new_by_models(found_animes.unwrap());
    animes.sort();
    return HttpResponse::Ok().json(ResponseBody { animes });
}

pub async fn get_by_season(data: AppData, path_params: web::Path<PathParams>) -> impl Responder {
    let found_animes = AnimeEntity::find()
        .filter(AnimeColumn::Year.eq(path_params.year))
        .filter(AnimeColumn::Season.eq(path_params.season.clone().unwrap()))
        .all(&data.db)
        .await;

    if found_animes.is_err() {
        error!("Failed to find animes from DB.");
        return HttpResponse::InternalServerError().finish();
    }

    let mut animes = StrictAnime::new_by_models(found_animes.unwrap());
    animes.sort();
    return HttpResponse::Ok().json(ResponseBody { animes });
}

pub async fn post(data: AppData, body_params: web::Json<BodyParams>) -> impl Responder {
    let new_animes = &body_params.animes;
    info!("Try to insert animes: {:?}", new_animes);

    let target_option_animes = StrictAnime::to_active_models(new_animes.clone());
    let include_none = target_option_animes
        .clone()
        .into_iter()
        .any(|a| a.is_none());

    if include_none {
        error!("Failed to convert animes: {:?}", new_animes);
        return HttpResponse::BadRequest().finish();
    }

    let target_animes = target_option_animes
        .into_iter()
        .map(|a| a.unwrap())
        .collect::<Vec<_>>();
    let insert_result = AnimeEntity::insert_many(target_animes).exec(&data.db).await;

    if insert_result.is_err() {
        error!("Failed to insert animes: {:?}", insert_result);
        return HttpResponse::BadRequest().finish();
    }

    return HttpResponse::Ok().finish();
}

pub async fn put(data: AppData, body_params: web::Json<BodyParams>) -> impl Responder {
    let animes = &body_params.animes;
    info!("Try to update animes: {:?}", animes);

    let mut updated_animes = vec![];

    for anime in animes {
        let new_option_anime = anime.clone().to_active_model();

        if new_option_anime.is_none() {
            let msg = "Failed to convert an anime";
            error!("{}: {:?}", msg, anime);
            return HttpResponse::BadRequest().json(ErrorResponseBody {
                reason: msg.to_string(),
                successful_animes: updated_animes,
                failed_anime: anime.to_owned(),
            });
        }

        let new_anime = new_option_anime.unwrap();
        let updated_anime = new_anime.update(&data.db).await;

        if updated_anime.is_err() {
            let msg = "Failed to update an anime";
            error!("{}: {:?} => {:?}", msg, anime, updated_anime);
            return HttpResponse::BadRequest().json(ErrorResponseBody {
                reason: msg.to_string(),
                successful_animes: updated_animes.clone(),
                failed_anime: anime.to_owned(),
            });
        }

        info!("Succeeded to update an anime: {:?}", anime);
        updated_animes.push(StrictAnime::new_by_model(updated_anime.unwrap()));
    }

    return HttpResponse::Ok().json(ResponseBody {
        animes: updated_animes,
    });
}

pub async fn delete(data: AppData, body_params: web::Json<BodyParams>) -> impl Responder {
    let animes = &body_params.animes;
    info!("Try to delete animes: {:?}", animes);

    let mut target_id_options = animes.iter().map(|a| a.id);
    let include_none = target_id_options.any(|a| a.is_none());

    if include_none {
        error!("Delete target anime.id is must be set: {:?}", animes);
        return HttpResponse::BadRequest().finish();
    }

    let target_ids = target_id_options.map(|a| a.unwrap()).collect::<Vec<_>>();
    info!("Delete target IDs: {:?}", target_ids);

    let delete_result = AnimeEntity::delete_many()
        .filter(AnimeColumn::Id.is_in(target_ids))
        .exec(&data.db)
        .await;

    if delete_result.is_err() {
        error!("Failed to delete animes: {:?}", delete_result);
        return HttpResponse::BadRequest().finish();
    }

    return HttpResponse::Ok().finish();
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};
    use rstest::*;
    use sea_orm::ActiveValue::{NotSet, Set};
    use sea_orm::DatabaseConnection;

    use entity::anime::ActiveModel as AnimeActiveModel;

    #[fixture]
    #[once]
    fn setup() {
        crate::logger::init_logger();
    }

    async fn clear_animes(db: &DatabaseConnection) {
        let _ = AnimeEntity::delete_many().exec(db).await;
    }

    async fn create_anime_1(db: &DatabaseConnection) {
        let anime = AnimeActiveModel {
            id: NotSet,
            year: Set(2023),
            season: Set("winter".to_string()),
            day: Set("Sun".to_string()),
            time: Set("00:00L00".to_string()),
            station: Set("station-1".to_string()),
            title: Set("anime-1".to_string()),
            recommend: Set(false),
        };
        let _ = anime.insert(db).await.unwrap();
    }

    #[rstest]
    #[actix_rt::test]
    async fn test_get(_setup: ()) {
        let app_state = AppState::init().await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state.clone()))
                .route("/animes", web::get().to(get)),
        )
        .await;

        create_anime_1(&app_state.db).await;

        let req = test::TestRequest::get().uri("/animes").to_request();
        println!("{:?}", req);
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert_eq!(resp.status(), StatusCode::OK);

        let body: ResponseBody = test::read_body_json(resp).await;
        assert_eq!(body.animes.len(), 1);

        clear_animes(&app_state.db).await;
    }

    #[rstest]
    #[actix_rt::test]
    async fn test_get_empty(_setup: ()) {
        let app_state = AppState::init().await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state.clone()))
                .route("/animes", web::get().to(get)),
        )
        .await;

        let req = test::TestRequest::get().uri("/animes").to_request();
        println!("{:?}", req);
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert_eq!(resp.status(), StatusCode::OK);

        let body: ResponseBody = test::read_body_json(resp).await;
        assert_eq!(body.animes.len(), 0);

        clear_animes(&app_state.db).await;
    }
}
