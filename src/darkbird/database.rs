use anymap::AnyMap;
use dashmap::{mapref::one::Ref, iter::Iter, DashSet};
use tokio::sync::mpsc::Sender;
use std::{hash::Hash, sync::Arc, time::Duration};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Storage, document::Document, Event};

use super::{SessionResult, storage_redis::RedisStorage};



pub struct Database {
    datastores: AnyMap
}

impl Database {
    

    pub fn open(datastores: AnyMap) -> Database {
        Database { datastores }
    }


    #[inline]        
    pub async fn subscribe<K, Doc>(&self, sender: Sender<Event<K, Doc>>) -> Result<(), SessionResult> 
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                datastore.subscribe(sender).await
            }
        }
    }

    #[inline]        
    pub async fn insert<K, Doc>(&self, key: K, doc: Doc) -> Result<(), SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                datastore.insert(key, doc).await
            }
        }
    }

    #[inline]        
    pub async fn remove<K, Doc>(&self, key: K) -> Result<(), SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                datastore.remove(key).await
            }
        }
    }


    
    #[inline]        
    pub fn gets<'a, K, Doc>(&self, list: Vec<&K>) -> Result<Vec<Ref<K, Doc>>, SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                let res = datastore.gets(list);
                Ok(res)
            }
        }
    }


    #[inline]        
    pub fn range<K, Doc>(&self, field_name: &str, from: String, to: String) -> Result<Vec<Ref<K, Doc>>, SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                let res = datastore.range(field_name, from, to);
                Ok(res)
            }
        }
    }



    #[inline]        
    pub fn lookup<K, Doc>(&self, key: &K) -> Result<Option<Ref<K, Doc>>, SessionResult> 
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                let res = datastore.lookup(key);
                Ok(res)
            }
        }
    }


    #[inline]        
    pub fn lookup_by_index<K, Doc>(&self, index_key: &str) -> Result<Option<Ref<K, Doc>>, SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                let res = datastore.lookup_by_index(index_key);
                Ok(res)
            }
        }
    }



    #[inline]        
    pub fn lookup_by_tag<K, Doc>(&self, tag: &str) -> Result<Vec<Ref<K, Doc>>, SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                let res = datastore.lookup_by_tag(tag);
                Ok(res)
            }
        }
    }



    #[inline]        
    pub fn fetch_view<K, Doc>(&self, view_name: &str) -> Result<Vec<Ref<K, Doc>>, SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                let res = datastore.fetch_view(view_name);
                Ok(res)
            }
        }
    }



    #[inline]        
    pub fn search<K, Doc>(&self, text: String) -> Result<Vec<Ref<K, Doc>>, SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                let res = datastore.search(text);
                Ok(res)
            }
        }
    }



    #[inline]        
    pub fn iter<K, Doc>(&self) -> Result<Iter<'_, K, Doc>, SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                let res = datastore.iter();
                Ok(res)
            }
        }
    }


    #[inline]        
    pub fn iter_index<K, Doc>(&self) -> Result<Iter<String, K>, SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                let res = datastore.iter_index();
                Ok(res)
            }
        }
    }


    #[inline]        
    pub fn iter_tags<K, Doc>(&self) -> Result<Iter<String, DashSet<K>>, SessionResult>
    where
        Doc: Serialize + DeserializeOwned + Clone + Send + 'static + Document,
        K:  Serialize
            + DeserializeOwned
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Clone
            + Send
            + Sync
            + 'static
    {
        match self.datastores.get::<Storage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                let res = datastore.iter_tags();
                Ok(res)
            }
        }
    }




    /// Just for redisstore engine
    #[inline]
    pub fn set<K, Doc>(&self, key: K, value: Doc, expire: Option<Duration>) -> Result<(), SessionResult>
    where
        Doc: Clone + Send + Sync + 'static,
        K:  Clone
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Send
            + 'static
    {
        match self.datastores.get::<RedisStorage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                datastore.set(key, value, expire);
                Ok(())
            }
        }
    }


    /// Just for redisstore engine
    #[inline]
    pub fn get<K, Doc>(&self, key: &K) -> Result<Option<Arc<Doc>>, SessionResult>
    where
        Doc: Clone + Send + Sync + 'static,
        K:  Clone
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Send
            + 'static
    {
        match self.datastores.get::<RedisStorage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                Ok(datastore.get(key))
            }
        }
    }


    /// Just for redisstore engine
    #[inline]
    pub fn del<K, Doc>(&self, key: &K) -> Result<(), SessionResult>
    where
        Doc: Clone + Send + Sync + 'static,
        K:  Clone
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Send
            + 'static
    {
        match self.datastores.get::<RedisStorage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                Ok(datastore.del(key))
            }
        }
    }



    /// Just for redisstore engine
    #[inline]
    pub fn set_nx<K, Doc>(&self, key: K, value: Doc, expire: Option<Duration>) -> Result<bool, SessionResult>
    where
        Doc: Clone + Send + Sync + 'static,
        K:  Clone
            + PartialOrd
            + Ord
            + PartialEq
            + Eq
            + Hash
            + Send
            + 'static
    {
        match self.datastores.get::<RedisStorage<K, Doc>>() {
            None => Err(SessionResult::DataStoreNotFound),
            Some(datastore) => {
                Ok(datastore.set_nx(key, value, expire))
            }
        }
    }



    



}