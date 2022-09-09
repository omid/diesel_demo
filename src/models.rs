use crate::schema::posts;
use crate::schema::sql_types::PostStatus;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::AsExpression;
use diesel::FromSqlRow;
use diesel::{deserialize, serialize};
use std::io::Write;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub status: PostStatusEnum,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub status: PostStatusEnum,
}

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, Clone)]
#[diesel(sql_type = PostStatus)]
pub enum PostStatusEnum {
    Draft,
    Published,
    Deleted,
}

impl ToSql<PostStatus, Pg> for PostStatusEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let res = match self {
            PostStatusEnum::Draft => "draft",
            PostStatusEnum::Published => "published",
            PostStatusEnum::Deleted => "deleted",
        };

        out.write_all(res.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<PostStatus, Pg> for PostStatusEnum {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let res = match bytes.as_bytes() {
            b"draft" => PostStatusEnum::Draft,
            b"published" => PostStatusEnum::Published,
            b"deleted" => PostStatusEnum::Deleted,
            _ => return Err("Unrecognized enum variant".into()),
        };
        Ok(res)
    }
}
