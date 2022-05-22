use types::{Hash256, Signature, Keypair, PublicKey};
use std::sync::Arc;
use crate::utils::error::DvfError;
use crate::node::dvfcore::BlsSignature;
use network::ReliableSender;
use std::net::SocketAddr;
use bytes::Bytes;
use tokio::sync::mpsc::{Receiver};
use std::collections::HashSet;
use downcast_rs::DowncastSync;
use futures::executor::block_on;
pub trait TOperator: DowncastSync + Sync + Send {
    fn sign(&self, msg: Hash256) -> Signature; 
    fn public_key(&self) -> Result<&PublicKey, DvfError>;
}
impl_downcast!(sync TOperator);

pub struct LocalOperator {
    pub voting_keypair: Arc<Keypair>
}

impl TOperator for LocalOperator {

    fn sign(&self, msg: Hash256) -> Signature {
        self.voting_keypair.sk.sign(msg)
    }

    fn public_key(&self) -> Result<&PublicKey, DvfError> {
        Ok(&self.voting_keypair.pk)
    }
}

impl LocalOperator {
    pub fn from_keypair(keypair: Arc<Keypair>) -> Self {
        Self {
            voting_keypair: keypair
        }
    }
}

pub struct HotStuffOperator {
    pub voting_keypair: Arc<Keypair>,
    pub network: ReliableSender,
    pub signature_address: SocketAddr,
    // bls_signature: Option<BlsSignature>,
}

impl TOperator for HotStuffOperator {

    fn sign(&self, msg: Hash256) -> Signature {
        self.voting_keypair.sk.sign(msg)
    }

    fn public_key(&self) -> Result<&PublicKey, DvfError> {
        Ok(&self.voting_keypair.pk)
    }
}

// hotstuff remote operator
impl HotStuffOperator {
    pub fn new(keypair: Arc<Keypair>, address: SocketAddr) -> Self {
        Self {
            voting_keypair: keypair,
            network: ReliableSender::new(),
            signature_address: address
        }
    }

    pub async fn request_signature(&mut self, msg: Hash256) -> Option<BlsSignature> {
        let receiver = self.network.send(self.signature_address, Bytes::from(msg.to_fixed_bytes().to_vec())).await;
        match receiver.await {
            Ok(data) => {
                println!("got = {:?}", data);
                match bincode::deserialize::<BlsSignature>(&data) {
                    Ok(bls_signature) =>{
                        Some(bls_signature)
                    },
                    Err(e) => {
                        None
                    }
                }
            },
            Err(_) => {
                println!("the sender dropped");
                None
            }
        }
    }

    /// send msg to network for consensus
    // pub async fn propose(&mut self, msg: Hash256) {
    //     // prefix id is fixed 
    //     // update
    //     let validator_vec : Vec<u8>= vec![50; 88];
    //     let validator_id = String::from_utf8(validator_vec).unwrap();
    //     let mut prefix_msg : Vec<u8> = Vec::new();
    //     prefix_msg.extend(validator_id.into_bytes());
    //     prefix_msg.extend(msg.to_fixed_bytes().to_vec());
    //     self.network.send(self.address, Bytes::from(prefix_msg)).await;
    // }

    // pub async fn wait_signature(&mut self) -> Vec<SignatureInfo>{
    //     let mut received = 0;
    //     let mut ids = HashSet::<u32>::new();
    //     let mut res = Vec::new();
    //     loop {
    //         if let Some(signature_info) = self.rx_signature.recv().await {
    //             if ids.contains(&signature_info.id)  {
    //                 continue;
    //             } else {
    //                 ids.insert(signature_info.id);
    //                 res.push(signature_info);
    //                 received+=1;
    //             }
    //             if received == 10 {
    //                 break;
    //             }
                    
    //         }
    //     }
    //     res
    // }
}

pub struct RemoteOperator {
    // [Zico]TODO: to be updated
    pub voting_keypair: Arc<Keypair>
}

impl TOperator for RemoteOperator {
    fn sign(&self, msg: Hash256) -> Signature { 
        // [Zico]TODO: request remote operator to sign and return the signature
        self.voting_keypair.sk.sign(msg)
    }

    fn public_key(&self) -> Result<&PublicKey, DvfError> {
        Err(DvfError::Unknown)  
    }
}
