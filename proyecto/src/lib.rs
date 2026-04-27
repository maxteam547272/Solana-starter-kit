use anchor_lang::prelude::*;

// USA TU ID: HUyHqUy3gpUT2YqNP8SVK9bP6SQ8QFN6FVd4M5f9k3RN
declare_id!("HUyHqUy3gpUT2YqNP8SVK9bP6SQ8QFN6FVd4M5f9k3RN");

#[program]
pub mod gaming_guild {
    use super::*;

    // CREATE: Inicializa el perfil del jugador usando una PDA
    pub fn registrar_jugador(ctx: Context<RegistrarJugador>, nombre: String) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil_jugador;
        perfil.nombre = nombre;
        perfil.nivel = 1;
        perfil.misiones_completadas = 0;
        perfil.autor = *ctx.accounts.usuario.key;
        msg!("Jugador registrado exitosamente");
        Ok(())
    }
    // UPDATE: Actualiza el estado (Misiones y Nivel)
    pub fn completar_mision(ctx: Context<ActualizarPerfil>) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil_jugador;
        perfil.misiones_completadas += 1;

        // Lógica de nivel: cada 3 misiones sube de nivel
        if perfil.misiones_completadas % 3 == 0 {
            perfil.nivel += 1;
            msg!("¡Subiste al nivel {}!", perfil.nivel);
        }
        Ok(())
    }

// UPDATE: Cambia el nombre (Demuestra edición de strings en PDA)
    pub fn editar_perfil(ctx: Context<ActualizarPerfil>, nuevo_nombre: String) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil_jugador;
        perfil.nombre = nuevo_nombre;
        Ok(())
    }

    // DELETE: Cierra la cuenta y recupera los SOL (Cierre de PDA)
    pub fn eliminar_perfil(_ctx: Context<EliminarPerfil>) -> Result<()> {
        msg!("Perfil eliminado y fondos recuperados");
        Ok(())
    }
}
#[account]
pub struct PerfilJugador {
    pub autor: Pubkey,             // 32 bytes
…        payer = usuario, 
        space = 8 + 32 + 40 + 2 + 2,
        seeds = [b"jugador", usuario.key().as_ref()],
        bump
    )]
    pub perfil_jugador: Account<'info, PerfilJugador>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActualizarPerfil<'info> {
    #[account(mut, has_one = autor)]
    pub perfil_jugador: Account<'info, PerfilJugador>,
    pub autor: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarPerfil<'info> {
    #[account(mut, has_one = autor, close = autor)]
    pub perfil_jugador: Account<'info, PerfilJugador>,
    pub autor: Signer<'info>,
}
