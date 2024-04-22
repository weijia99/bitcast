use std::collections::BTreeMap;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::data::log_record::LogRecordPos;

use super::Indexer;


pub struct Btree{
    tree:Arc<RwLock<BTreeMap<Vec<u8>,LogRecordPos>>>,
}
// add new function to create a new Btree
impl Btree{
    pub fn new()->Self{
        Self{
            tree:Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}
impl Indexer for Btree{
    fn get(&self,key:Vec<u8>)->Option<LogRecordPos> {
        // todo!()
        // obtain read guard
        let read_guard = self.tree.read();
        read_guard.get(&key).copied()

    }

    fn put(&self,key:Vec<u8>,pos:LogRecordPos)->bool {
        // todo!()
        // obtain write guard
        let mut write_guard = self.tree.write();
        write_guard.insert(key,pos);
        true
    }

    fn delete(&self,key:Vec<u8>)->bool {
        // todo!()
        // obtain write guard
        let mut write_guard = self.tree.write();
        let remove_res =write_guard.remove(&key);
        // check if the key is removed
        remove_res.is_some()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_btree_put(){
        let btree = Btree::new();
        let res =btree.put("".as_bytes().to_vec(),LogRecordPos{filed_id:1,offset:1});
        assert_eq!(res,true);
    }
    #[test]
    fn test_btree_get(){
        let btree = Btree::new();
        btree.put("".as_bytes().to_vec(),LogRecordPos{filed_id:1,offset:1});
        let res =btree.get("".as_bytes().to_vec());
        assert!(res.is_some());
        assert_eq!(res.unwrap().filed_id,1);
        assert_eq!(res.unwrap().offset,1);
    }
    #[test]
    fn test_btree_delete(){
        let btree = Btree::new();
        btree.put("".as_bytes().to_vec(),LogRecordPos{filed_id:1,offset:1});
        let res =btree.delete("".as_bytes().to_vec());
        assert_eq!(res,true);
        let res =btree.delete("aaa".as_bytes().to_vec());
        assert_eq!(res,false);

    }
}