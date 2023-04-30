use near_sdk::log;
use crate::*;

pub trait Founders{
    //change functions
    fn dis_founders(&mut self);
    fn update_founders_account(&mut self,account_id:AccountId)->AccountId;
    fn founders_minted_tokens(&self)->U128;
    
}

#[near_bindgen]
impl Founders for Contract{
    fn dis_founders(&mut self){
        self.only_owner();
        // 1 MONTH UNIX TIME = 2,629,746,000
        // ms for 2 mint 120000
        //token supply check  5000000000
        let total_tokens= U128(1000000000000000000000000000000000);
        require!(self.total_supply<=total_tokens.into(),"All tokens have minted");
        // total chess tournment grants amount
        let total_founders_tokens= U128(80000000000000000000000000000000);
        // checking if all founders grants have minted.
        require!(self.founders_minted_tokens<total_founders_tokens.into(),"All founders grants already minted");
        //granting 8.25% tokens every 3 months 
        if env::block_timestamp_ms()>=self.founders_timestamp+7889238000
        // for testing purpose 10 second
        // if env::block_timestamp_ms()>=self.founders_timestamp+10000
        {
            self.founders_timestamp=env::block_timestamp_ms();
            // 8.25% of founders Grants
            let supply:U128=U128(6666667000000000000000000000000);
            self.internal_deposit(&self.founders_ac.clone(), supply.into());
            //adding to total supply
            self.total_supply+=supply.0;
            
            // Emit an event showing that the FTs were minted
            FtMint {
                owner_id: &self.founders_ac.clone(),
                amount: &supply,
                memo: Some("Founders's Token Minted After Passing 1 Month Duration")
            }
            .emit();
            // month increment of Treasury Protocol grants minting
            self.founders_months+=1;
            // increment in minted tokens of founders
            self.founders_minted_tokens+=supply.0;
        }
        // if timestamp not elapsed
        else {
            panic!("You cannot unlock Founder's Tokens before unlocking duration");
        }
        
    }
    
    //  UPDATE TREASURY ACCOUNT (ONLY ADMIN)
    fn update_founders_account(&mut self,account_id:AccountId)->AccountId{
        self.only_owner();
        self.founders_ac=account_id;
        log!("Updated founders Grant Account Id : {}",self.founders_ac.clone());
        self.founders_ac.clone()
    }

    // VIEW FUNCTION
    fn founders_minted_tokens(&self)->U128{
        self.founders_minted_tokens.into()
    }
}