Construyamos un sistema de perfil de usuario que demuestre cómo los métodos getter proporcionan acceso de lectura controlado a los datos privados.

Crearás dos archivos para organizar tu código:

profile.rs: Define una estructura UserProfile con dos campos privados: username (String) y age (u32). La estructura en sí debe ser pública, pero los campos deben permanecer privados. Implementa un constructor público new que reciba un nombre de usuario y una edad. Después, añade dos métodos getter:
username: devuelve una referencia al nombre de usuario (&String)
age: devuelve el valor de la edad (u32)
main.rs: Incluye tu módulo de perfil, crea un UserProfile y usa los métodos getter para imprimir la información del usuario.
Recuerda que los getter en Rust normalmente llevan el mismo nombre que el campo: no se necesita el prefijo get_. Para los campos String, devuelve una referencia para evitar copias innecesarias.

Tu salida debe seguir este formato exacto:

Username: {username}
Age: {age}
Por ejemplo, si creas un perfil con el nombre de usuario "alice" y la edad 25, la salida sería:

Username: alice
Age: 25
Recibirás dos entradas: el nombre de usuario y la edad.
