use anchor_lang::prelude::*;

// USA TU ID: HUyHqUy3gpUT2YqNP8SVK9bP6SQ8QFN6FVd4M5f9k3RN
declare_id!("HUyHqUy3gpUT2YqNP8SVK9bP6SQ8QFN6FVd4M5f9k3RN");

#[program]
pub mod gaming_guild {
    use super::*;

    // CREATE: Inicializa el perfil del jugador usando una PDA
…    #[account(mut, has_one = autor, close = autor)]
    pub perfil_jugador: Account<'info, PerfilJugador>,
    pub autor: Signer<'info>,
}
