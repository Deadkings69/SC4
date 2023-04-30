use near_sdk::log;
use crate::*;

pub trait TreasuryProtocol{
    //change functions
    fn dis_treasury_protocol(&mut self);
    fn update_treasury_protocol_account(&mut self,account_id:AccountId)->AccountId;
    fn treasury_minted_tokens(&self)->U128;
    
}

#[near_bindgen]
impl TreasuryProtocol for Contract{
    fn dis_treasury_protocol(&mut self){
        self.only_owner();
        // 1 MONTH UNIX TIME = 2,629,746,000
        // ms for 2 mint 120000
        //token supply check  5000000000
        let total_tokens= U128(1000000000000000000000000000000000);
        require!(self.total_supply<=total_tokens.into(),"All tokens have minted");
        // total treasury grants amount
        let total_treasury_tokens= U128(120000000000000000000000000000000);
        // checking if all treasury grants have minted.
        require!(self.treasury_minted_tokens<total_treasury_tokens.into(),"All treasury grants already minted");
        //granting tokens for first 3 months
        // for testing purpose 10 second
        // 1 month = 2,629,746,000
        if env::block_timestamp_ms()>=self.t_timestamp+2629746000 &&self.treasury_months<4
        {
            self.t_timestamp=env::block_timestamp_ms();
            // 5% of Treasury Protocol Grants
            let supply:U128=U128(6000000000000000000000000000000);
            self.internal_deposit(&self.treasury_protocol_ac.clone(), supply.into());
            //adding to total supply
            self.total_supply+=supply.0;
            
            // Emit an event showing that the FTs were minted
            FtMint {
                owner_id: &self.treasury_protocol_ac.clone(),
                amount: &supply,
                memo: Some("Treasury Protocol's Token Minted After Passing 1 Month Duration")
            }
            .emit();
            // month increment of Treasury Protocol grants minting
            self.treasury_months+=1;
            // total minted tokens into treasury protocol
            self.treasury_minted_tokens+=&supply.0;
        }
        // In this else another If Condition
        else {
            // checking if all community grants have minted.
            require!(self.treasury_minted_tokens<total_treasury_tokens.into(),"All treasury grants already minted");
            // granting after passing every 3 months
            // 3 month = 7889238000
            if env::block_timestamp_ms()>=self.t_timestamp+7889238000 &&self.treasury_months>=4
            // for testing purpose 10 second
            // if env::block_timestamp_ms()>=self.t_timestamp+10000 &&self.treasury_months>=4
        {
            self.t_timestamp=env::block_timestamp_ms();
            let supply:U128=U128(6000000000000000000000000000000);
            self.internal_deposit(&self.treasury_protocol_ac.clone(), supply.into());
            //adding to total supply
            self.total_supply+=supply.0;
            
            // Emit an event showing that the FTs were minted
            FtMint {
                owner_id: &self.treasury_protocol_ac.clone(),
                amount: &supply,
                memo: Some("Treasury Protocol's Tokens Minted After passing 3 Months Duration")
            }
            .emit();
            //month increment
            self.treasury_months+=1;
            // total minted tokens into treasury protocol
            self.treasury_minted_tokens+=&supply.0;
        }
        else {
            panic!("You cannot unlock Treasury Protocol's Tokens before unlocking duration");
        }
        }
        
    }
    
    //  UPDATE TREASURY ACCOUNT (ONLY ADMIN)
    fn update_treasury_protocol_account(&mut self,account_id:AccountId)->AccountId{
        self.only_owner();
        self.treasury_protocol_ac=account_id;
        log!("Updated Treasury Account Id : {}",self.treasury_protocol_ac.clone());
        self.treasury_protocol_ac.clone()
    }

    // VIEW FUNCTION
    fn treasury_minted_tokens(&self)->U128{
        self.treasury_minted_tokens.into()
    }
}