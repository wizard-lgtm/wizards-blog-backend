use mongodb::{bson, Collection};
use mongodb::{self, Client, Database};
use std::env;
use mongodb::bson::{doc, Bson, Uuid};
use mongodb::error::Result as MongoResult;
use mongodb::options::ClientOptions;
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



struct NoteService {
    collection: Collection<Note>,
}
impl NoteService {
    pub async fn new(uri: &str, db_name: &str, collection_name: &str) -> MongoResult<Self> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let collection = client.database(db_name).collection(collection_name);
        Ok(Self { collection })
    }

    pub async fn create_note(&self, title: String, content: String, tags: Vec<Tags>) -> MongoResult<Uuid> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let note = Note {
            id: Uuid::new(),
            title,
            content,
            created_at: now,
            updated_at: now,
            tags,
        };
        let _id = note.id.clone();
        self.collection.insert_one(note).await?;
        Ok(_id)
    }

    pub async fn delete_note(&self, id: Uuid) -> MongoResult<()> {
        self.collection.delete_one(doc! { "id": id }).await?;
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
}