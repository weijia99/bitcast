pub mod btree;
use crate::data::log_record::LogRecordPos;
pub trait Indexer{
    fn get(&self,key:Vec<u8>)->Option<LogRecordPos>;
    fn put(&self,key:Vec<u8>,pos:LogRecordPos)->bool;
    fn delete(&self,key:Vec<u8>)->bool;
}