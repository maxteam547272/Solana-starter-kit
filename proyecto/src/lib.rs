use anchor_lang::prelude::*;

declare_id!("TU_NUEVA_ID_AQUI"); // Recuerda darle a 'Build' para que se genere una nueva

#[program]
pub mod gaming_quest {
    use super::*;

    pub fn iniciar_jugador(ctx: Context<IniciarJugador>, nombre: String) -> Result<()> {
        let jugador = &mut ctx.accounts.jugador;
        jugador.nombre = nombre;
        jugador.nivel = 1;
        jugador.xp = 0;
        msg!(
            "¡Jugador {} creado! Nivel: {}",
            jugador.nombre,
            jugador.nivel
        );
        Ok(())
    }

    pub fn ganar_experiencia(ctx: Context<SubirNivel>) -> Result<()> {
        let jugador = &mut ctx.accounts.jugador;
        jugador.xp += 10;
        if jugador.xp >= 30 {
            jugador.nivel += 1;
            jugador.xp = 0;
            msg!("¡SUBISTE DE NIVEL! Ahora eres nivel {}", jugador.nivel);
        } else {
            msg!("Ganaste 10 de XP. Te falta para el siguiente nivel.");
        }
        Ok(())
    }
}

#[account]
pub struct Jugador {
    pub nombre: String, 
    pub nivel: u16,     
    pub xp: u16,        
}

#[derive(Accounts)]
pub struct IniciarJugador<'info> {
    #[account(init, payer = user, space = 8 + 40 + 2 + 2)]
    pub jugador: Account<'info, Jugador>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubirNivel<'info> {
    #[account(mut)]
    pub jugador: Account<'info, Jugador>,
}
