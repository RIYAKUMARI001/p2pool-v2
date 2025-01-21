// Copyright (C) 2024 [Kulpreet Singh]
//
//  This file is part of P2Poolv2
//
// P2Poolv2 is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free 
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// P2Poolv2 is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS 
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with 
// P2Poolv2. If not, see <https://www.gnu.org/licenses/>. 

use tokio::sync::oneshot;
use crate::node::messages::Message;
use crate::shares::ShareBlock;
use crate::shares::miner_message::MinerWorkbase;
use std::error::Error;

/// Commands for communication between node handle and actor
pub enum Command{
    /// Command telling node's event loop to send share to network
    SendGossip(Vec<u8>, oneshot::Sender<()>),
    /// Command telling node's event loop to send message to a specific peer
    SendToPeer(libp2p::PeerId, Message, oneshot::Sender<()>),
    /// Command to get a list of connected peers
    GetPeers(oneshot::Sender<Vec<libp2p::PeerId>>),
    /// Command to shutdown node
    Shutdown(oneshot::Sender<()>),
    /// Command to add share to the chain
    AddShare(ShareBlock, oneshot::Sender<Result<(), Box<dyn Error + Send + Sync>>>),
    /// Command to store workbase in the node's database
    StoreWorkbase(MinerWorkbase, oneshot::Sender<Result<(), Box<dyn Error + Send + Sync>>>),
}