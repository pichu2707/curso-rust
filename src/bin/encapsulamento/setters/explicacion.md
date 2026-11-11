Construyamos un sistema de termostato que demuestre cómo los setters pueden aplicar reglas para mantener los datos válidos. Tu termostato tendrá un ajuste de temperatura que solo se podrá modificar dentro de un rango seguro.

Crearás dos archivos para organizar tu código:

thermostat.rs: Define una estructura Thermostat con un campo temperature privado (i32). La estructura debe ser pública, pero el campo debe permanecer privado para protegerlo de valores no válidos. Implementa:
Un constructor new que reciba una temperatura inicial
Un getter temperature que devuelva la temperatura actual
Un setter set_temperature que solo acepte valores entre 10 y 30 (inclusive). Si el valor está fuera de este rango, la temperatura debe permanecer sin cambios.
main.rs: Incluye tu módulo del termostato, crea un termostato y demuestra la validación del setter intentando establecer varias temperaturas e imprimiendo el resultado después de cada intento.
El setter debe ignorar silenciosamente los valores no válidos: sin mensajes de error, simplemente debe mantener la temperatura actual sin cambios cuando alguien intente establecerla fuera del rango válido.

Tu salida debe seguir exactamente este formato:

Initial: {temperature}
After setting to {attempted_value}: {temperature}
After setting to {attempted_value}: {temperature}
Por ejemplo, si creas un termostato a 20, después intentas establecerlo en 25 (válido) y después en 50 (no válido), la salida sería:

Initial: 20
After setting to 25: 25
After setting to 50: 25
Recibirás tres entradas: la temperatura inicial, la primera temperatura que se intentará establecer y la segunda temperatura que se intentará establecer.



