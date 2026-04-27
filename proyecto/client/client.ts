import * as anchor from "@coral-xyz/anchor";

describe("Gaming Guild CRUD", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = pg.program;

  // Derivamos la dirección de la PDA
  const [perfilPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("jugador"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Ejecutando ciclo CRUD completo en Solana", async () => {
    // 1. CREATE - Registrar al jugador
    await program.methods.registrarJugador("Ruth_Pro").accounts({
      perfilJugador: perfilPDA,
      usuario: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("¡Perfil creado exitosamente en la PDA!");

    // 2. UPDATE - Completar una misión
    await program.methods.completarMision().accounts({
      perfilJugador: perfilPDA,
      autor: provider.wallet.publicKey,
    }).rpc();
    console.log("¡Misión completada y XP actualizada!");

    // 3. DELETE - Eliminar perfil (Cierre de cuenta)
    await program.methods.eliminarPerfil().accounts({
      perfilJugador: perfilPDA,
      autor: provider.wallet.publicKey,
    }).rpc();
    console.log("¡Perfil eliminado y cuenta cerrada correctamente!");
  });
});
