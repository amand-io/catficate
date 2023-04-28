use lazy_static::lazy_static;
use std::sync::Mutex;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    record::Key, AddProviderOk, GetProvidersOk, GetRecordOk, Kademlia, KademliaEvent, PeerRecord,
    PutRecordOk, QueryResult, Quorum, Record,
};
use libp2p::{
    identity, mdns, noise,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Transport,
};

lazy_static! {

    pub static ref MY_GLOBAL_KAD: Mutex<Kademlia<MemoryStore>> = {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        let store = MemoryStore::new(local_peer_id);
        Mutex::new(Kademlia::new(local_peer_id, store))
    }
}

pub fn init_global_KAD(new : Kademlia<MemoryStore>) {
    let mut var = MY_GLOBAL_KAD.lock().unwrap();
    *var = new;
}

pub fn store_global_KAD(id String) -> Kademlia<MemoryStore> {
    let mut var = MY_GLOBAL_KAD.lock().unwrap();
    return var
}

pub fn get_global_KAD() -> Kademlia<MemoryStore> {
    let mut var = MY_GLOBAL_KAD.lock().unwrap();
    return var
}
