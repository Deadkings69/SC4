use crate::*;

pub trait MintingToken{
    //change functions
    fn mint(&mut self,supply:U128);
    
}

#[near_bindgen]
impl MintingToken for Contract{
   fn mint(&mut self,supply:U128){
    self.only_owner();
    // restriction not mint more than  1 billion
    let total_tokens=U128(1000000000000000000000000000000000);
    require!(self.total_supply<=total_tokens.0,"All supply minted. You cannot mint more tokens.");
    if self.total_supply+supply.0>total_tokens.0{
        panic!("You cannot mint more than {} tokens",self.total_supply-total_tokens.0);
    }
    // restriction to mint only 0.2 billion token
    let _total_tokens=U128(200000000000000000000000000000000);
    if self.public_minting<_total_tokens.0-4 && self.public_minting+supply.0<_total_tokens.0-4
    {

        self.internal_deposit(&self.owner.clone(), supply.into());
        
        // Emit an event showing that the FTs were minted
        FtMint {
            owner_id: &self.community_ac.clone(),
            amount: &supply,
            memo: Some("Token Minted")
        }
        .emit();
        // Incrementing to total supply
        self.total_supply+=supply.0;
        // Incrementing
        self.public_minting+=supply.0;
    }
    else {
        panic!("Already Minted 0.2 billion tokens or your amount is exceding from 0.2 billion tokens");
    }

   }
}