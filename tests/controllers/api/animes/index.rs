use actix_web::http::StatusCode;
use actix_web::{test, web, App};
use rstest::*;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::DatabaseConnection;
use sea_orm::{ActiveModelTrait, EntityTrait};

use entity::anime::ActiveModel as AnimeActiveModel;
use entity::anime::Entity as AnimeEntity;

use okkey_api::app_state::AppState;
use okkey_api::controllers::api::animes::index::{get, ResponseBody};

#[fixture]
#[once]
fn setup() {
    okkey_api::logger::init_logger();
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
    println!("{req:?}");
    let resp = test::call_service(&app, req).await;
    println!("{resp:?}");
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
    println!("{req:?}");
    let resp = test::call_service(&app, req).await;
    println!("{resp:?}");
    assert_eq!(resp.status(), StatusCode::OK);

    let body: ResponseBody = test::read_body_json(resp).await;
    assert_eq!(body.animes.len(), 0);

    clear_animes(&app_state.db).await;
}
