use mongodb::{bson, Collection};
use mongodb::{self, Client, Database};
use serde_json::json;
use std::env;
use mongodb::bson::{doc, Bson, Document, Uuid};
use mongodb::error::{Error, Result as MongoResult};
use mongodb::options::{ClientOptions, FindOptions};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::{Note, Tags};



pub async fn db_connect(uri: String) -> Database{

    let db_name: String = env::var("DB_NAME").expect("Environment variable DB_NAME is not set or invalid");


    let connection: Client = match mongodb::Client::with_uri_str(uri).await {
        Ok(c) => c, 
        Err(e) => {
            panic!("Some error happened while connecting to database: {:?}", e);
        }
    }; 

    let db = connection.database(&db_name);

    return db;

}




pub struct NoteService {
    collection: Collection<Note>,
}
impl NoteService {
    pub async fn new(db: &Database) -> MongoResult<Self> {
        
        let collection_name = "notes";
        let collection =db.collection(&collection_name); 
        Ok(Self { collection })
    }

    pub async fn create_note(&self, title: String, content: String, tags: Vec<Tags>) -> MongoResult<Uuid> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let note = Note {
            id: Uuid::new(),
            title,
            content: Some(content),
            created_at: now,
            updated_at: now,
            tags,
        };
        let _id = note.id.clone();
        self.collection.insert_one(note).await?;
        Ok(_id)
    }
    pub async fn delete_note(&self, id: Uuid) -> MongoResult<()> {
        let note = self.find_note_by_id(id).await?;

        if note.is_none() {
            return Err(Error::custom(json!({"meesage": "Resource not found"})));
        }

        let result = self.collection.delete_one(doc! { "id": id }).await?;

        if result.deleted_count == 0 {
            return Err(Error::custom({"Failed to delete resource"}));
        }

        Ok(())
    }


       pub async fn update_note(
        &self,
        id: Uuid,
        new_title: Option<String>,
        new_content: Option<String>,
        new_tags: Option<Vec<Tags>>,
    ) -> MongoResult<()> {
        let mut update_doc = doc! {};
        if let Some(title) = new_title {
            update_doc.insert("title", title);
        }
        if let Some(content) = new_content {
            update_doc.insert("content", content);
        }
        if let Some(tags) = new_tags {
            update_doc.insert("tags", bson::to_bson(&tags)?);
        }

        let now : u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        update_doc.insert("created_at ", Bson::from(now as i64));
        update_doc.insert("updated_at",Bson::from(now as i64));

        Ok(())
    }

    pub async fn find_note_by_id(&self, id: Uuid) -> MongoResult<Option<Note>> {
        self.collection.find_one(doc! { "id": id }).await
    }

    pub async fn find_notes(&self, tags: Option<Vec<Tags>>) -> MongoResult<Vec<Note>> {
        let filter = if let Some(tags) = tags {
            doc! { "tags": { "$in": bson::to_bson(&tags)? } }
        } else {
            doc! {}
        };

        let mut cursor = self.collection.find(filter).await?;
        let mut notes = Vec::new();
        while cursor.advance().await? {
            let note: Note = cursor.deserialize_current()?;
            notes.push(note);
        }
        Ok(notes)
    }
    pub async fn get_note_by_title(&self, title: String)->MongoResult<Option<Note>>{
            let filter = doc! {"title": title};
            let result = self.collection.find_one(filter).await?;
            
            Ok(result)
    }
    pub async fn get_all_notes_by_page(&self, page: usize) -> MongoResult<Vec<Note>> {
        if page == 0 {
            return MongoResult::Err(Error::custom(
                "Page number must be greater than 0".to_string(),
            ));
        }

        let page_size = 10;
        let skip = (page - 1) * page_size; // Skip for pagination
        let filter = doc! {}; // No filter for fetching all notes
        
        let mut cursor = self.collection.find(filter)
            .skip(skip as u64)
            .limit(page_size as i64)
            .projection(doc! { "content": 0 }) // Exclude the `content` field
        .await?;
        let mut notes = Vec::new();

        while cursor.advance().await? {
            let note: Note = cursor.deserialize_current()?;
            notes.push(note);
        }

        Ok(notes)
    }

    
}