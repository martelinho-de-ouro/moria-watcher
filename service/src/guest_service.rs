use ::entity::{guest, guest::Entity as Guest};
use prelude::Uuid;
use sea_orm::*;

pub struct GuestService;

impl GuestService {
    pub async fn create(
        db: &DbConn,
        form_data: guest::Model,
    ) -> Result<guest::ActiveModel, DbErr> {
        guest::ActiveModel {
            name: Set(form_data.name.to_owned()),
            email: Set(form_data.email.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update(
        db: &DbConn,
        id: Uuid,
        form_data: guest::Model,
    ) -> Result<guest::Model, DbErr> {
        let guest: guest::ActiveModel = Guest::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find.".to_owned()))
            .map(Into::into)?;

        guest::ActiveModel {
            id: guest.id,
            name: Set(form_data.name.to_owned()),
            email: Set(form_data.email.to_owned()),
            phone: Set(form_data.phone.to_owned()),
            address: Set(form_data.address.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
        let guest: guest::ActiveModel = Guest::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find.".to_owned()))
            .map(Into::into)?;

        guest.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Guest::delete_many().exec(db).await
    }

    pub async fn find_by_id(
        db: &DbConn,
        id: Uuid,
    ) -> Result<Option<guest::Model>, DbErr> {
        Guest::find_by_id(id).one(db).await
    }

    pub async fn find_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<guest::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Guest::find()
            .order_by_asc(guest::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
