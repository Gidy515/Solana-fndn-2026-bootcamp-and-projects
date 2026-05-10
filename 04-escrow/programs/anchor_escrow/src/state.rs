use anchor_lang::prelude::*;
 
#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct Escrow {
    /// Arbitrary seed used to derive the PDA for this escrow.
    /// Ensures uniqueness (e.g. maker can create multiple escrows).
    /// Typically passed from the client when creating the escrow.
    pub seed: u64,

    /// The creator of the escrow (the one offering tokens).
    /// This is the authority who initializes and can potentially cancel the escrow.
    pub maker: Pubkey,

    /// The mint address of the token the maker is DEPOSITING.
    /// Example: USDC mint if maker is offering USDC.
    pub mint_a: Pubkey,

    /// The mint address of the token the maker expects in return.
    /// Example: SOL-wrapped or another SPL token.
    pub mint_b: Pubkey,

    /// The amount of `mint_b` tokens the maker wants to RECEIVE.
    /// This defines the "price" or exchange expectation.
    /// Important: this does NOT store how much was deposited,
    /// only what the maker expects in return.
    pub receive: u64,

    /// PDA bump used to derive the escrow account address.
    /// Stored so the program can re-derive the PDA later (for signing, validation, etc).
    pub bump: u8,
}