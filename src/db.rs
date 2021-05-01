use std::result;

use crate::todo::List;
use crate::Error;

use futures::StreamExt; // So the mongodb Cursor<T> has a next().
use mongodb::{bson::doc, bson::oid::ObjectId, bson::Document, Database};

// Simplify return results.
type Result<T> = result::Result<T, Error>;

pub struct Db {
    db: Database,
}

impl Db {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, name: &str) -> Result<List> {
        let l = List {
            id: bson::oid::ObjectId::new().to_hex(),
            name: name.to_string(),
            items: vec![],
        };

        self.db
            .collection("list")
            .insert_one(l.clone(), None)
            .await
            .map_err(|e| Error::MongoDB(e))?;

        Ok(l)
    }

    pub async fn get(&self, id: &str) -> Result<List> {
        let id = ObjectId::with_string(id).map_err(|e| Error::Oid(e))?;
        let filter = doc! {"_id": id };

        let find = self
            .db
            .collection::<List>("list")
            .find_one(filter, None)
            .await
            .map_err(|e| Error::MongoDB(e))?;

        match find {
            Some(l) => Ok(l),
            None => Err(Error::NotFound),
        }
    }

    pub async fn all(&self) -> Result<Vec<List>> {
        let mut ll: Vec<List> = Vec::new();
        let filter = doc! {};

        let mut cursor = self
            .db
            .collection::<List>("list")
            .find(filter, None)
            .await
            .map_err(|e| Error::MongoDB(e))?;
        while let Some(doc) = cursor.next().await {
            ll.push(doc.map_err(|e| Error::MongoDB(e))?);
        }

        return Ok(ll);
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let id = ObjectId::with_string(id).map_err(|e| Error::Oid(e))?;
        let filter = doc! {"_id": id};

        let results = self
            .db
            .collection::<Document>("list")
            .delete_one(filter, None)
            .await
            .map_err(|e| Error::MongoDB(e))?;

        // Check for no deletion to return not found.
        if results.deleted_count > 0 {
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    pub async fn update(&self, list: List) -> Result<()> {
        let id = ObjectId::with_string(&list.id).map_err(|e| Error::Oid(e))?;
        let filter = doc! {"_id": id};
        let results = self
            .db
            .collection::<List>("list")
            .replace_one(filter, list, None)
            .await
            .map_err(|e| Error::MongoDB(e))?;

        // Check for update to return not found.
        if results.modified_count > 0 {
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }
}
