use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Anime::Schema, Anime::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Anime::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Anime::Year).integer().not_null())
                    .col(ColumnDef::new(Anime::Season).string().not_null())
                    .col(ColumnDef::new(Anime::Day).string().not_null())
                    .col(ColumnDef::new(Anime::Time).string().not_null())
                    .col(ColumnDef::new(Anime::Station).string().not_null())
                    .col(ColumnDef::new(Anime::Title).string().not_null())
                    .col(ColumnDef::new(Anime::Recommend).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Anime::Schema, Anime::Table))
                    .to_owned(),
            )
            .await
    }
}

enum Anime {
    Schema,
    Table,
    Id,
    Year,
    Season,
    Day,
    Time,
    Station,
    Title,
    Recommend,
}

impl Iden for Anime {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => "gokabot",
                Self::Table => "animes",
                Self::Id => "id",
                Self::Year => "year",
                Self::Season => "season",
                Self::Day => "day",
                Self::Time => "time",
                Self::Station => "station",
                Self::Title => "title",
                Self::Recommend => "recommend",
            }
        )
        .unwrap();
    }
}
