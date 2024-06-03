use anchor_lang::prelude::*;
use std::{collections::HashMap, iter::Map};

declare_id!("5nrPYHucWFU2kbaVtzDyuRDmny4McbNwxDTqQSYrFV9s");

#[program]
pub mod solana_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // init value

        Ok(())
    }

    ///////////////////////////
    ////// SYSTEM ACTION //////
    ///////////////////////////
    pub fn create_token(token_id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Token {
    token: Pubkey,
    settleTime: u64,
    settleDuration: u64,
    settleRate: u64,
    status: u64,
}

pub struct Offer {
    offerType: u64,
    tokenId: u64,
    exToken: Pubkey,
    amount: u64,
    value: u64,
    collateral: u64,
    filledAmount: u64,
    status: u64,
    offeredBy: Pubkey,
    fullMatch: bool,
}

pub struct Order {
    offerId: u64,
    amount: u128,
    seller: Pubkey,
    buyer: Pubkey,
    status: u64,
}

pub struct Config {
    pledgeRate: u64,
    feeRefund: u64,
    feeSettle: u64,
    feeWallet: Pubkey,
}
//#[derive(Debug, Default, Clone)]
pub struct PreMarketStorage {
    acceptedTokens: HashMap<Pubkey, bool>,
    tokens: HashMap<u64, Token>,
    offers: HashMap<u64, Offer>,
    lastOfferId: u64,
    orders: HashMap<u64, Order>,
    lastOrderId: u64,
    owner: Pubkey,
    config: Config,
}

pub const WEI6: u64 = 1000_000;
pub const OFFER_BUY: u64 = 1;
pub const OFFER_SELL: u64 = 2;

// Status
// Offer status
pub const STATUS_OFFER_OPEN: u64 = 1;
pub const STATUS_OFFER_FILLED: u64 = 2;
pub const STATUS_OFFER_CANCELLED: u64 = 3;

// Order Status
pub const STATUS_ORDER_OPEN: u64 = 1;
pub const STATUS_ORDER_SETTLE_FILLED: u64 = 2;
pub const STATUS_ORDER_SETTLE_CANCELLED: u64 = 3;
pub const STATUS_ORDER_CANCELLED: u64 = 3;

// token status
pub const STATUS_TOKEN_ACTIVE: u64 = 1;
pub const STATUS_TOKEN_INACTIVE: u64 = 2;
pub const STATUS_TOKEN_SETTLE: u64 = 3;

impl PreMarketStorage {
    // initialize config
    pub fn set_config(&mut self) {
        self.config.pledgeRate = WEI6;
        self.config.feeRefund = WEI6 / 200; // 0.5%
        self.config.feeSettle = WEI6 / 40; // 2.5%
        self.config.feeWallet = self.owner; //owner
    }

    pub fn create_token(&mut self, token_id: u64, settle_duration: u64) -> Result<()> {
        let _token = self.tokens.get_mut(&token_id).unwrap();
        _token.settleDuration = settle_duration;
        _token.status = STATUS_TOKEN_ACTIVE;

        Ok(())
    }

    pub fn get_token()-> Result<(PreMarketStorage)>{

        Ok()
    }
}

#[derive(Accounts)]
pub struct Initialize {
    //pub preMarketStorage: Box<Account<'info, PreMarketStorage>>,
}
