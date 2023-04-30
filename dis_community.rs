use near_sdk::log;
use crate::*;

pub trait Community{
    //change functions
    fn dis_community(&mut self);
    fn update_community_account(&mut self,account_id:AccountId)->AccountId;
    fn community_minted_tokens(&self)->U128;
    
}

#[near_bindgen]
impl Community for Contract{
    fn dis_community(&mut self){
        self.only_owner();
        // 1 MONTH UNIX TIME = 2,629,746,000
        // ms for 2 mint 120000
        //token supply check  5000000000
        let total_tokens= U128(1000000000000000000000000000000000);
        require!(self.total_supply<=total_tokens.into(),"All tokens have minted");
        // total community grants 5 % of 1 billion token supply
        let total_community_tokens= U128(50000000000000000000000000000000);
        // checking if all community grants have minted.
        require!(self.community_minted_tokens<total_community_tokens.into(),"All community grants already minted");
        //granting 5% tokens every 3 months 
        if env::block_timestamp_ms()>=self.com_timestamp+7889238000
        // for testing purpose 10 sec
        // if env::block_timestamp_ms()>=self.com_timestamp+10000
        {
            
            self.com_timestamp=env::block_timestamp_ms();
            // 5% of Community Grants
            let supply:U128=U128(2500000000000000000000000000000);
            self.internal_deposit(&self.community_ac.clone(), supply.into());
            //adding to total supply
            self.total_supply+=supply.0;
            
            // Emit an event showing that the FTs were minted
            FtMint {
                owner_id: &self.community_ac.clone(),
                amount: &supply,
                memo: Some("Community's Token Minted After Passing 1 Month Duration")
            }
            .emit();
            // month increment of Community grants minting
            self.community_months+=1;
            // total minted tokens into Community
            self.community_minted_tokens+=&supply.0;
        }
        // if timestamp not elapsed
        else {
            panic!("You cannot unlock Community's Tokens before unlocking duration");
        }
        
    }
    
    //  UPDATE TREASURY ACCOUNT (ONLY ADMIN)
    fn update_community_account(&mut self,account_id:AccountId)->AccountId{
        self.only_owner();
        self.community_ac=account_id;
        log!("Updated Community Grant Account Id : {}",self.community_ac.clone());
        self.community_ac.clone()
    }

    // VIEW FUNCTION
    fn community_minted_tokens(&self)->U128{
        self.community_minted_tokens.into()
    }
}