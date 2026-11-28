¡Te damos la bienvenida al proyecto Mascota Virtual! Durante las próximas lecciones, crearás una simulación completa de una mascota que reúne todo lo que has aprendido sobre las estructuras, los métodos, los módulos y la encapsulación.

¡Comencemos a crear tu Mascota Virtual! En este primer paso, configurarás los cimientos creando una estructura Pet en su propio módulo con una encapsulación adecuada.

Organizarás tu código en dos archivos:

pet.rs: Define tu estructura Pet con tres campos privados: name (String), energy (u32) y hunger (u32). Implementa una función asociada pública new que reciba un nombre y cree una mascota con energy establecido en 100 y hunger establecido en 0. Añade también un método getter público name que devuelva una referencia al nombre de la mascota.
main.rs: Declara el módulo pet e incorpora Pet al ámbito. Lee un nombre de la entrada, crea una mascota nueva con ese nombre e imprime un mensaje de bienvenida.
Tu mensaje de bienvenida debe seguir este formato:

Welcome, {name}!
Por ejemplo, si la entrada es Buddy, la salida debería ser:

Welcome, Buddy!
Recuerda que la estructura debe ser pública para poder usarse desde main.rs, pero los campos deben permanecer privados: solo se podrá acceder a ellos mediante los métodos que proporciones.
