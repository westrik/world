use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{object_types, object_types::dsl::object_types as all_object_types};

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct ObjectType {
    pub id: i32,
    pub api_id: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "objectName")]
    pub object_name: String,
    pub description: Option<String>,
    #[serde(rename = "barcodePrefix")]
    pub barcode_prefix: Option<String>,
    #[serde(rename = "barcodeLength")]
    pub barcode_length: Option<i32>,
}

#[derive(Insertable, Debug)]
#[table_name = "object_types"]
pub struct ObjectTypeCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub object_name: String,
    pub description: Option<String>,
    pub barcode_prefix: Option<String>,
    pub barcode_length: Option<i32>,
}
impl ObjectTypeCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<ObjectType, ApiError> {
        info!("creating object type: {:#?}", self);
        Ok(diesel::insert_into(object_types::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

#[allow(clippy::option_option)]
#[derive(AsChangeset, Debug)]
#[table_name = "object_types"]
pub struct ObjectTypeUpdateSpec {
    // TODO: use trigger to set updated_at automatically
    pub updated_at: DateTime<Utc>,
    pub object_name: Option<String>,
}
impl ObjectTypeUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<ObjectType, ApiError> {
        info!("updating object type {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_object_types
                .filter(object_types::api_id.eq(&api_id))
                .filter(object_types::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<ObjectType>(conn)
        .map_err(ApiError::DatabaseError)?)
    }
}

impl ObjectType {
    pub fn find_all_for_user(
        conn: &PgConnection,
        session: Session,
    ) -> Result<Vec<ObjectType>, ApiError> {
        let items: Vec<ObjectType> = all_object_types
            .filter(object_types::user_id.eq(session.user_id))
            .load(conn)
            .map_err(ApiError::DatabaseError)?;
        Ok(items)
    }

    pub fn find_by_api_id(
        conn: &PgConnection,
        session: Session,
        api_id: String,
    ) -> Result<ObjectType, ApiError> {
        let item: ObjectType = all_object_types
            .filter(object_types::user_id.eq(session.user_id))
            .filter(object_types::api_id.eq(api_id))
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?;
        Ok(item)
    }

    pub fn create(
        conn: &PgConnection,
        session: Session,
        object_name: String,
        description: Option<String>,
        // TODO: improve these types
        barcode_prefix: Option<String>,
        barcode_length: Option<i32>,
    ) -> Result<ObjectType, ApiError> {
        let new_object_type = ObjectTypeCreateSpec {
            api_id: generate_resource_identifier(ResourceType::ObjectType),
            user_id: session.user_id,
            object_name,
            description,
            barcode_prefix,
            barcode_length,
        };
        new_object_type.insert(conn)
    }

    pub fn update(
        conn: &PgConnection,
        session: Session,
        api_id: String,
        object_name: Option<String>,
    ) -> Result<ObjectType, ApiError> {
        let object_type = ObjectTypeUpdateSpec {
            updated_at: Utc::now(),
            object_name,
        }
        .update(conn, api_id.clone(), session.user_id)?;
        Ok(object_type)
    }
}
