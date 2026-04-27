¿De qué trata esto?
Este proyecto lo hice para simular la lógica básica de un videojuego en la blockchain. La idea es que un jugador pueda crear su perfil (un "Guerrero") y que su progreso se guarde de forma real en Solana. No es solo un mensaje de texto; aquí estamos manejando datos que persisten, como el nombre del jugador, su nivel y cuánta experiencia (XP) lleva acumulada.

Lo que aprendí y apliqué (El lado técnico):
Para que este proyecto funcionara, me enfoqué en tres conceptos clave que vimos en el Bootcamp:

CRUD Completo: * Create: Uso una función para inicializar al jugador.

Update: Tengo una lógica que suma XP y, cuando llegas a cierto puntaje (30 XP), te sube de nivel automáticamente. Básicamente es un "Level Up" programado en el contrato.

PDA (Program Derived Address): Esto fue de lo más retador. Usé las cuentas de Solana para que cada jugador tenga su propio espacio de memoria. Usé init para crear la cuenta y definí el space necesario para que no nos falte (ni sobre) memoria al guardar el nombre y los números.

Seguridad con Anchor: El contrato está blindado para que solo el usuario que firma la transacción pueda pagar por su creación de cuenta y modificar su estado.

¿Cómo probarlo?
Si quieres ver que sí jala, solo hay que:

Hacer el Build y Deploy en el Playground de Solana (usando la Devnet).

Correr el test que dejé en la carpeta client.

En la terminal vas a ver cómo se crea el usuario "Ruth_Master" y cómo, al ganar batallas, el sistema te va avisando cuánta experiencia llevas hasta que subes de nivel.

Nota personal: Batallé un poco con el IDL en el navegador, pero al final la lógica en Rust quedó sólida y las transacciones se confirman correctamente en la red de pruebas.

¿Por qué esto suena a estudiante?
Usas palabras como "literal", "batallé", "jaló", "blindado".

Mencionas que algo fue "retador" (a los profes les encanta saber que te costó trabajo pero lo lograste).
