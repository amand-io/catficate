use async_std::io;
use futures::{prelude::*, select};
use libp2p::core::upgrade::Version;
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
use std::error::Error;
use tokio::runtime::Runtime;

// We create a custom network behaviour that combines Kademlia and mDNS.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
struct MyBehaviour {
    kademlia: Kademlia<MemoryStore>,
    mdns: mdns::async_io::Behaviour,
}

#[allow(clippy::large_enum_variant)]
enum MyBehaviourEvent {
    Kademlia(KademliaEvent),
    Mdns(mdns::Event),
}

impl From<KademliaEvent> for MyBehaviourEvent {
    fn from(event: KademliaEvent) -> Self {
        MyBehaviourEvent::Kademlia(event)
    }
}

impl From<mdns::Event> for MyBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        MyBehaviourEvent::Mdns(event)
    }
}

struct Api {
    swarm: libp2p::Swarm<MyBehaviour>,
}

impl Api {
    fn start() -> Result<Self, Box<dyn Error>> {
        // Create a random key for ourselves.
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        let transport = tcp::async_io::Transport::default()
            .upgrade(Version::V1Lazy)
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::YamuxConfig::default())
            .boxed();

        
        // Create a swarm to manage peers and events.
        let swarm = {
            // Create a Kademlia behaviour.
            let store = MemoryStore::new(local_peer_id);
            let kademlia = Kademlia::new(local_peer_id, store);
            let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id)?;
            let behaviour = MyBehaviour { kademlia, mdns };
            SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build()
        };
        Ok(Self { swarm })
    }

    fn store_data(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let record = Record {
            key: Key::new(&key),
            value,
            publisher: None,
            expires: None,
        };
        self.swarm.behaviour_mut().kademlia.put_record(record, Quorum::One);
    }

    fn get_data(&mut self, key: Vec<u8>) -> Option<Vec<u8>> {
        let mut rt = Runtime::new().unwrap();
        let key = Key::new(&key);
        self.swarm.behaviour_mut().kademlia.get_record(key);
        let result = rt.block_on(
            self.swarm
                .for_each(|event| match event {
                    KademliaEvent::OutboundQueryCompleted { result, .. } => match result {
                        QueryResult::GetRecord(Ok(result)) => {
                            return Ok(result.records.first().map(|r| r.value.clone()))
                        }
                        _ => return Ok(None),
                    },
                    _ => Ok(()),
                })
                .map_err(|_| ()),
        );
        result.ok()
    }
}