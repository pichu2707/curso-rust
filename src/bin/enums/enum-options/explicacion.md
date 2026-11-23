Vamos a crear un sistema de perfiles de usuario que gestione con elegancia los datos opcionales mediante métodos de Option. Puede que los usuarios tengan un apodo o puede que no. Tu sistema gestionará ambos casos con elegancia, sin coincidencias explícitas.

Crearás dos archivos para organizar tu código:

profile.rs: Define una estructura pública Profile con dos campos: username (String) y nickname (Option<String>). Implementa:
Un constructor new que reciba un nombre de usuario y un apodo opcional
Un método display_name que devuelva el apodo si existe o, como alternativa, el nombre de usuario: usa unwrap_or para gestionarlo
Un método formatted_nickname que devuelva un Option<String> con el apodo entre corchetes (como "[CoolNick]") si existe, o None si no hay ningún apodo: usa map para transformar el valor
main.rs: Incluye el módulo de perfiles y crea dos perfiles: uno con apodo y otro sin él. Muestra ambos métodos para cada perfil para demostrar cómo los métodos de Option gestionan de forma diferente los valores presentes y ausentes.
Para el método formatted_nickname, usa el formato de depuración ({:?}) al imprimir, ya que devuelve un Option.

Tu salida debe seguir este formato exacto y mostrar ambos perfiles:

Display name: {name}
Formatted nickname: {option}
Display name: {name}
Formatted nickname: {option}
Por ejemplo, con un perfil para el nombre de usuario "alice" y el apodo "Ace", y otro para el nombre de usuario "bob" sin apodo, la salida sería:

Display name: Ace
Formatted nickname: Some("[Ace]")
Display name: bob
Formatted nickname: None
Recibirás tres entradas: el primer nombre de usuario, el apodo del primer usuario y el segundo nombre de usuario (que no tiene apodo).
