use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    self, Mint, Token2022, InterfaceAccount, MintTo, Burn, TransferChecked,
};
use spl_token_2022::extension::ExtensionType;
use solana_program::pubkey::Pubkey;

/// --- Oracle Integration Start ---
use pyth_sdk_solana::load_price_feed_from_account_info;
use chainlink_solana as chainlink;
/// --- Oracle Integration End ---

/// --- On-Chain Swap Integration Start ---
use spl_token_swap_interface::{
    Swap,
    SwapTokenAccount,
    SwapPool,
};
/// --- On-Chain Swap Integration End ---

/// --- Bridge Module Integration Start ---
pub mod bridge;
pub use bridge::*;
/// --- Bridge Module Integration End ---

/// --- Futures/Perpetual Module Integration Start ---
pub mod futures;
pub use futures::*;
/// --- Futures/Perpetual Module Integration End ---

/// --- Autonomous Control Module Integration Start ---
pub mod autonomy;
pub use autonomy::*;
/// --- Autonomous Control Module Integration End ---

declare_id!("YourProgramIdHere");

#[program]
pub mod optimized_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.total_minted = 0;
        config.max_supply = 9_000_000_000;
        config.transfer_tax_bps = 250; // 2.5%

        let extensions = [
            ExtensionType::MintCloseAuthority,
            ExtensionType::TransferFeeConfig,
            ExtensionType::TransferHook,
            ExtensionType::NonTransferable,
        ];

        let decimals = 6;
        require!(
            ctx.accounts.mint.decimals == decimals,
            ErrorCode::InvalidDecimals
        );

        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, requested_amount: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let remaining_supply = config.max_supply.checked_sub(config.total_minted).ok_or(ErrorCode::Overflow)?;

        let mint_amount = if requested_amount > remaining_supply {
            remaining_supply
        } else {
            requested_amount
        };

        require!(mint_amount > 0, ErrorCode::MaxSupplyReached);

        /// --- Oracle Integration Start ---
        let sol_usd_price = get_normalized_price(&ctx.accounts.pyth_sol_usd, &ctx.accounts.chainlink_sol_usd)?;
        let usdt_usd_price = get_normalized_price(&ctx.accounts.pyth_usdt_usd, &ctx.accounts.chainlink_usdt_usd)?;

        let diff = if sol_usd_price > usdt_usd_price {
            sol_usd_price - usdt_usd_price
        } else {
            usdt_usd_price - sol_usd_price
        };

        let deviation_percent = (diff * 10_000) / ((sol_usd_price + usdt_usd_price) / 2); // basis point

        require!(
            deviation_percent <= 400,
            ErrorCode::OraclePriceDeviationTooHigh
        );
        /// --- Oracle Integration End ---

        /// --- Autonomous Control Hook Start ---
        autonomy::enforce_dynamic_limits(ctx.accounts.clock.unix_timestamp, mint_amount)?;
        /// --- Autonomous Control Hook End ---

        let seeds = &[b"mint_auth", &[ctx.bumps.mint_authority]];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            signer,
        );

        token_interface::mint_to(cpi_ctx, mint_amount)?;
        config.total_minted = config.total_minted.checked_add(mint_amount).ok_or(ErrorCode::Overflow)?;

        Ok(())
    }

    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        );
        token_interface::burn(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64, decimals: u8) -> Result<()> {
        let config = &ctx.accounts.config;
        let tax_bps = config.transfer_tax_bps;

        let tax = amount.checked_mul(tax_bps.into()).unwrap().checked_div(10_000).unwrap();
        let amount_after_tax = amount.checked_sub(tax).unwrap();

        let cpi_ctx_tax = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.treasury.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        token_interface::transfer_checked(cpi_ctx_tax, tax, decimals)?;

        let cpi_ctx_transfer = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        token_interface::transfer_checked(cpi_ctx_transfer, amount_after_tax, decimals)?;

        Ok(())
    }

    pub fn swap_tokens(ctx: Context<SwapTokens>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_swap_program.to_account_info(),
            Swap {
                swap: ctx.accounts.swap.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                user_transfer_authority: ctx.accounts.user_transfer_authority.to_account_info(),
                source: ctx.accounts.source.to_account_info(),
                destination: ctx.accounts.destination.to_account_info(),
                pool_token_a: ctx.accounts.pool_token_a.to_account_info(),
                pool_token_b: ctx.accounts.pool_token_b.to_account_info(),
                pool_mint: ctx.accounts.pool_mint.to_account_info(),
                fee_account: ctx.accounts.fee_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        );

        spl_token_swap_interface::swap(cpi_ctx, amount_in, minimum_amount_out)?;
        Ok(())
    }

    pub fn send_cross_chain(ctx: Context<SendCrossChain>, destination_chain: u16, recipient: Vec<u8>, amount: u64) -> Result<()> {
        bridge::send_cross_chain(ctx, destination_chain, recipient, amount)
    }

    pub fn receive_cross_chain(ctx: Context<ReceiveCrossChain>, source_chain: u16, source_address: Vec<u8>, nonce: u64, payload: Vec<u8>) -> Result<()> {
        bridge::receive_cross_chain(ctx, source_chain, source_address, nonce, payload)
    }

    pub fn open_futures_position(ctx: Context<OpenFuturesPosition>, collateral_amount: u64, leverage: u8, direction: u8) -> Result<()> {
        futures::open_position(ctx, collateral_amount, leverage, direction)
    }

    pub fn close_futures_position(ctx: Context<CloseFuturesPosition>) -> Result<()> {
        futures::close_position(ctx)
    }
}
