¡Construyamos un sistema de casillero seguro que reúna todo lo que has aprendido sobre la encapsulación! Tu casillero protegerá su contenido con un código secreto: solo alguien que conozca el código podrá ver qué hay dentro.

Crearás dos archivos para organizar tu código:

locker.rs: Define una estructura Locker con dos campos privados: code (u32) y contents (String). La estructura debe ser pública, pero ambos campos deben permanecer privados para proteger la seguridad del casillero. Implementa:
Un constructor new que reciba un código y un contenido iniciales
Un método set_code que permita cambiar el código del casillero
Un método get_contents que reciba un código intentado; si coincide con el código del casillero, devuelve el contenido; de lo contrario, devuelve "Access denied"
main.rs: Incorpora tu módulo del casillero, crea un casillero y demuestra sus funciones de seguridad intentando acceder al contenido con códigos correctos e incorrectos.
El método get_contents debe devolver un String, ya sea el contenido real cuando el código sea correcto o el mensaje de denegación cuando sea incorrecto. Este es tu getter condicional que valida el acceso antes de revelar datos privados.

Tu salida debe seguir este formato exacto:

Attempt with {code}: {result}
Code changed
Attempt with {code}: {result}
Por ejemplo, si creas un casillero con el código 1234 y el contenido "Gold coins", después intentas usar el código 0000, cambias el código a 5678 e intentas usar 5678, la salida sería:

Attempt with 0000: Access denied
Code changed
Attempt with 5678: Gold coins
Recibirás cinco entradas: el código inicial, el contenido, el primer código intentado, el nuevo código que se establecerá y el segundo código intentado.
