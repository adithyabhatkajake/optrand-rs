use std::sync::Arc;
use tokio_util::time::DelayQueue;
use crate::{Context, Event};
use types::ProtocolMsg;
use tokio::time::Duration;


impl Context {
    /// Reacts to a new epoch
    /// When we have a new epoch, do the following:
    /// 1) Send a new PVSS vector to the current leader
    /// 2a) If leader, start making an aggregate block and propose the block
    /// 2b) If not, wait for a block from the leader, and use timeouts appropriately
    pub async fn new_epoch(&mut self, dq: &mut DelayQueue<Event>) {
        log::info!("Epoch {} ended, waiting for another epoch", self.epoch);
        self.epoch += 1;
        // Reset variables for this epoch
        self.epoch_reset();
        self.epoch_timer = self.epoch_timer + tokio::time::Duration::from_millis(11*self.config.delta);
        dq.insert(Event::EpochEnd, Duration::from_millis(11*self.config.delta));
        dq.insert(Event::ProposeTimeout, Duration::from_millis(4*self.config.delta));
        // Update leader of this epoch
        self.last_leader = self.next_leader();
        log::debug!("Sending PVSS Vector to the next leader {}", self.last_leader);
        
        // Send a new PVSS vector to the leader
        let pvec = self.config.pvss_ctx.generate_shares(&self.my_secret_key, &mut crypto::std_rng());
        // If I am not the leader send a fresh sharing to the current leader
        if self.last_leader != self.config.id {
            // Send (v,c,\pi)
            self.net_send.send((self.last_leader, 
                Arc::new(
                    ProtocolMsg::RawEpochPVSSSharing(
                        pvec)
                    )
                )
            ).unwrap();
            // Send C_r'(B_l) to the leader
            self.net_send.send(
                (self.last_leader, Arc::new(
                    ProtocolMsg::RawStatus(
                        self.epoch-1,
                        self.highest_height,
                        self.highest_cert.as_ref().clone()
                    )
                ))
            ).unwrap();
            return;
        } 
        
        // I am the leader
        self.last_leader_epoch = self.epoch; // This is the last epoch I was a leader of
        // First push my own sharing to the next proposal
        self.pvss_shares.push(pvec);
        self.pvss_indices.push(self.config.id);
        // Do I need to wait 2\Delta before proposing?
        if self.highest_height < self.epoch-1 {
            dq.insert(Event::Propose, Duration::from_millis(self.config.delta*2));
            return;
        }
        // Do I already have the latest status message? ANS: yes
        self.do_propose(dq).await;
    }
}