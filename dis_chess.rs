use near_sdk::log;
use crate::*;

pub trait ChessTournment{
    //change functions
    fn dis_chess(&mut self);
    fn update_chess_account(&mut self,account_id:AccountId)->AccountId;
    fn chess_minted_tokens(&self)->U128;
    
}

#[near_bindgen]
impl ChessTournment for Contract{
    fn dis_chess(&mut self){
        self.only_owner();
        // 1 MONTH UNIX TIME = 2,629,746,000
        // ms for 2 mint 120000
        //token supply check  5000000000
        let total_tokens= U128(1000000000000000000000000000000000);
        require!(self.total_supply<=total_tokens.into(),"All tokens have minted");
        // total chess tournment grants amount 5% of total supply
        let total_chess_tokens= U128(50000000000000000000000000000000);
        // checking if all chess grants have minted.
        require!(self.chess_minted_tokens<total_chess_tokens.into(),"All chess grants already minted");
        //granting 5% tokens every 2 months = 5259492000
        if env::block_timestamp_ms()>=self.chess_timestamp+5259492000
        // for testing purpose 10 second
        // if env::block_timestamp_ms()>=self.chess_timestamp+10000
        {
            self.chess_timestamp=env::block_timestamp_ms();
            // 5% of chess Grants
            let supply:U128=U128(2500000000000000000000000000000);
            self.internal_deposit(&self.chess_ac.clone(), supply.into());
            //adding to total supply
            self.total_supply+=supply.0;
            
            // Emit an event showing that the FTs were minted
            FtMint {
                owner_id: &self.chess_ac.clone(),
                amount: &supply,
                memo: Some("Chess Tournment's Token Minted After Passing 1 Month Duration")
            }
            .emit();
            // month increment of Treasury Protocol grants minting
            self.chess_months+=1;
            // increment in total chess tournment minted tokens
            self.chess_minted_tokens+=&supply.0;
        }
        // if timestamp not elapsed
        else {
            panic!("You cannot unlock Chess Tournment's Tokens before unlocking duration");
        }
        
    }
    
    //  UPDATE TREASURY ACCOUNT (ONLY ADMIN)
    fn update_chess_account(&mut self,account_id:AccountId)->AccountId{
        self.only_owner();
        self.chess_ac=account_id;
        log!("Updated chess Grant Account Id : {}",self.chess_ac.clone());
        self.chess_ac.clone()
    }
    // VIEW FUNCTION
    fn chess_minted_tokens(&self)->U128{
        self.chess_minted_tokens.into()
    }
}