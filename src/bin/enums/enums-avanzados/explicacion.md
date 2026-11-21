Construirás un procesador de comandos que maneje distintos tipos de comandos de usuario. Cada comando contiene datos diferentes, y tu procesador usará la coincidencia de patrones para extraer esos datos y producir respuestas adecuadas.

Crearás dos archivos para organizar tu código:

command.rs: Define un enum público Command con tres variantes:
Say: contiene un único mensaje String (sintaxis de tupla)
Move: contiene campos con nombre: direction (String) y steps (u32)
Calculate: contiene dos valores i32 que representan los números que se sumarán (sintaxis de tupla)
Después, implementa un método execute que use match para desestructurar cada variante y devolver un String que describa la acción realizada.
main.rs: Incluye tu módulo de comandos, crea una instancia de cada variante de comando usando las entradas proporcionadas y llama al método execute en cada una para imprimir los resultados.
Tu método execute debe desestructurar cada variante para acceder a sus datos y devolver mensajes con estos formatos exactos:

Para Say: Saying: {message}
Para Move: Moving {steps} steps {direction}
Para Calculate: {a} + {b} = {sum}
Tu salida debe mostrar el resultado de cada comando en su propia línea:

Saying: {message}
Moving {steps} steps {direction}
{a} + {b} = {sum}
Por ejemplo, con un mensaje de say "Hello world", un movimiento de 5 pasos hacia "north" y un cálculo de 10 y 25, la salida sería:

Saying: Hello world
Moving 5 steps north
10 + 25 = 35
Recibirás cinco entradas: el mensaje de say, la dirección, el número de pasos y los dos números que se calcularán.
