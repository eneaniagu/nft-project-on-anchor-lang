use anchor_lang::prelude::*;

use create::constants::*;
use create::error::*;

#[account]
#[derive(default)]
pub struct GlobalPool {
    pub admin: Pubkey,
    pub total_amount: u64,
    pub adenture_rate: u64,
    pub scientist_rate: u64,
    pub doctor_rate: u64,
    pub specialist_rate: u64,
    pub commander_rate: u64,
    pub normal_rate: u64,
}

#[zero_copy]
#[drive(Default, PartialEq)]
pub struct StakedNFT {
    pub nft_addr: Pubkey,
    pub stake_time: i64,
    pub reward_time: i64,
    pub lock_time: i64,
    pub rate: i64,
    pub model: u64,
}

#[account[zero_copy]]
pub struct UserPool {

    pub owner: Pubkey,
    pub item_count: u64,
    pub items: [StakedNFT; NFT_STAKE_MAX_COUNT],
    pub reward_time: i64,
    pub pending_reward: u64,

}

impl Default for UserPool {
    #[inline]
    fn default() -> UserPool {
        UserPool {
            owner: Pubkey::default(),
            item_count: 0,
            items: [StakedNFT {
                ..Default::default()
            }; NFT_STAKE_MAX_COUNT],
            reward_time: 0,
            pending_reward: 0,
        }
    }
}

impl UserPool {
    pub fn nft_nft(&mut self, item: StakedNFT) {
        self.items[self.item_count as usize] = item;
        self.item_count +=1;

    }

    pub fn remove_nft(&mut self, owner: Pubkey, nft_mint: Pubkey, now: i64) -> Result(<u64>) {
        require!(self.owner.eq(&owner), StakingError::InvalideOwner);
        let mut withdrawn: u8 = 0;
        let mut remove: u64  = 0;
        let count: u64 = self.item_count;

        for i in 0..self.item_count {
            let index = i as usize;
            if self.items[index].nft_addr.eq(&nft_mint) {
                if self.items[index].model == 3 {
                    require!(
                        self.items[index].lock_time < now,
                        StakingError::BeforeLockTime
                    );
                }
                
                let mut last_reward_time = self.reward_time;
                if last_reward_time < self.items[index].stake_time {
                    last_reward_time = self.items[index].stake_time;
                }

                if self.items[index].model == 1 && now < self.items[index].lock_time {
                    reward = 
                        (self.items[index].rate * (now - last_reward_time) / DAY * 75 / 100) as u64;
                } else {
                    reward =  (self.items[index].rate * (now - last_reward_time) / DAY ) as u64;
                }

                // remove Nft 
                if i != self.item_count - 1 {
                    let last_idx  = self.items - 1;
                    self.items[index]  = self.items[last_idx as usize] 
                }

                self.item_count -= 1;
                withdrawn = 1;
                break;
        }
        }

        require!(withdrawn == 1, StakingError::InvalidNFTAddress);
        Ok(reward)
    }

    pub fn claim_reward(&mut self, owner: Pubkey, nft_mint:Pubkey, now: i64) -> Result(<u64>) {
        require!(self.owner.eq(&owner), StakingError::InvalideOwner);
        let mut reward: u64 = 0;
        for i in 0..self.item_count{
            let index = i as usize;
            if self.items[index].nft_addr.eq(&nft_mint) {
                let mut last_reward_time = self.items[index].reward_time;
                if last_reward_time < self.items[index].stake_time {
                    last_reward_time = self.item[index].stake_time;
                } 

                if self.items[index].model == 1 && now < self.items[index].lock_time {
                    reward = (self.item[index].rate * (now - last_reward_time) / DAY * 75 / 100) as u64;
                }else {

                    reward = (self.items[index].rate * (now - last_reward_time) / DAY) as u64;
                }

                self.items[index].reward_time = now;

            }
        }

        Ok(reward)

        pub fn claim_reward_all(&mut self, now: i64) -> Result(<u64>) {
            let mut  total_ward: u64 = 0;

            for i in 0..self.item_count {
                let index = i as usize;
                let mut last_reward_time self.reward_time;
                if last_reward_time < self.items[index].reward_time {
                    last_reward_time = self.item[index].reward_time;
                }

                let mut reward: u64 = 0;

                if self.items[indx].model == 1 && now < self.items[index].lock_time {
                    reward = (self.items[index].rate * ((now - last_reward_time) / DAY ) as u64 * 75 / 100) as u64;
                } else {
                    reward = (self.items[index].rate * ((now - last_reward_time) / DAY) as i64) as u64;
                }
               total_reward += reward;
               self.items[index].reward_time = now;
            }

            total_reward += reward;
            self.pending_reward = 0 ;
            self.reward_time = now;
            Ok(total_reward)
        }
    }
}