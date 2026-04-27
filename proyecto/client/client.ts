import * as anchor from "@coral-xyz/anchor";

describe("Sistema de RPG Solana", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = pg.program;
  const jugadorKeypair = anchor.web3.Keypair.generate();

  it("¡Crea un nuevo Guerrero!", async () => {
    await program.methods
      .iniciarJugador("Ruth_Master")
      .accounts({
        jugador: jugadorKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([jugadorKeypair])
      .rpc();

    const cuenta = await program.account.jugador.fetch(
      jugadorKeypair.publicKey
    );
    console.log("Nombre del Heroe:", cuenta.nombre);
    console.log("Nivel Inicial:", cuenta.nivel.toString());
  });

  it("¡Gana batallas y sube de nivel!", async () => {
    await program.methods
      .ganarExperiencia()
      .accounts({
        jugador: jugadorKeypair.publicKey,
      })
      .rpc();

    const cuenta = await program.account.jugador.fetch(
      jugadorKeypair.publicKey
    );
    console.log("XP actual:", cuenta.xp.toString());
  });
});
