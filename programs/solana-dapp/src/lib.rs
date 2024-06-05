use anchor_lang::{prelude::*, solana_program::clock};
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
    pub fn create_token(ctx: Context<TokenStore>) -> Result<()> {
        let new_token = ctx.accounts.token.create_token(1000);
        Ok(())
    }
}

pub const WEI6: u64 = 1_000_000;
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

#[account]
#[derive(Default)]
pub struct Token {
    token: Pubkey,
    settleTime: i64,
    settleDuration: u64,
    settleRate: u64,
    status: u64,
}

impl Token {
    // ONLY ADMIN
    pub fn create_token(&mut self, settle_duration: u64) -> Result<()> {
        // check token already exists
        self.settleDuration = settle_duration;
        self.status = STATUS_TOKEN_ACTIVE;

        Ok(())
    }

    pub fn token_to_settle_phase(&mut self, tokenAddress: Pubkey, settleRate: u64) -> Result<()> {
        self.token = tokenAddress;
        self.settleTime = clock::Clock::get()?.unix_timestamp;
        self.settleRate = settleRate;
        self.status = STATUS_TOKEN_SETTLE;
        Ok(())
    }

    pub fn tokenToggleActivation(&mut self) -> Result<()> {
        let to_status;
        if self.status == STATUS_TOKEN_ACTIVE {
            to_status = STATUS_TOKEN_INACTIVE;
        } else {
            to_status = STATUS_TOKEN_ACTIVE;
        };
        self.status = to_status;
        Ok(())
    }

    pub fn getToken(&self) -> Result<Token> {
        let token = Token {
            token: self.token,
            settleTime: self.settleTime,
            settleDuration: self.settleDuration,
            settleRate: self.settleRate,
            status: self.status,
        };
        Ok(token)
    }
}

#[account]
#[derive(Default)]
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

impl Offer {
    pub fn newOffer(&mut self) -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct Order {
    offerId: u64,
    amount: u128,
    seller: Pubkey,
    buyer: Pubkey,
    status: u64,
}

impl Order {
    pub fn newOrder(&mut self) -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct Config {
    pledgeRate: u64,
    feeRefund: u64,
    feeSettle: u64,
    feeWallet: Pubkey,
}
impl Config {
    pub fn newConfig(&mut self) -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct PreMarket {
    acceptedTokens: HashMap<Pubkey, bool>,
    tokens: HashMap<u64, Token>,
    offers: HashMap<u64, Offer>,
    lastOfferId: u64,
    orders: HashMap<u64, Order>,
    lastOrderId: u64,
    owner: Pubkey,
    config: Config,
}

impl PreMarket {
    // initialize config
    pub fn set_config(&mut self) {
        self.config.pledgeRate = WEI6;
        self.config.feeRefund = WEI6 / 200; // 0.5%
        self.config.feeSettle = WEI6 / 40; // 2.5%
        self.config.feeWallet = self.owner; //owner
    }

    pub fn create_pre_market(
        &mut self,
        owner: Pubkey,
        token: Token,
        offer: Offer,
        order: Order,
        config: Config,
    ) -> Result<()> {
        self.acceptedTokens.insert(owner, true);
        self.tokens.insert(1, token);
        self.offers.insert(self.lastOfferId + 1, offer);
        self.lastOfferId = self.lastOfferId + 1;

        self.orders.insert(self.lastOrderId + 1, order);
        self.lastOrderId = self.lastOrderId + 1;
        self.owner = owner;
        self.config = config;

        Ok(())
    }

    pub fn getPreMarket(&mut self) -> Result<PreMarket> {
        let preMarketStorage = PreMarket {
            acceptedTokens: self.acceptedTokens.clone(),
            tokens: self.tokens.clone(),
            offers: self.offers.clone(),
            lastOfferId: self.lastOfferId,
            orders: self.orders.clone(),
            lastOrderId: self.lastOrderId,
            owner: self.owner,
            config: self.config.clone(),
        };
        Ok(preMarketStorage)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    // #[account(mut)]
    // pub preMarket: Account<'info, PreMarket>,
}
#[derive(Accounts)]
pub struct TokenStore<'info> {
    #[account(mut)]
    pub token: Account<'info, Token>,
}
