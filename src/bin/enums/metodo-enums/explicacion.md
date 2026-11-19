Construirás un sistema de semáforo que utiliza una enumeración con métodos para describir el estado actual de la señal. Cada color del semáforo llevará información sobre cuánto dura, y un método proporcionará una descripción legible para las personas.

Crearás dos archivos para organizar tu código:

traffic.rs: Define una enumeración pública TrafficLight con tres variantes:
Red: contiene un u32 que representa los segundos de espera
Yellow: contiene un u32 que representa los segundos de espera
Green: contiene un u32 que representa los segundos para avanzar
Después implementa un método status que devuelva un String describiendo el semáforo. Usa match dentro del método para gestionar cada variante e incluir la duración en el mensaje.
main.rs: Incorpora tu módulo de tráfico, crea instancias de cada variante del semáforo y llama al método status en cada una para imprimir las descripciones.
El método status debe devolver mensajes con estos formatos exactos:

Para Red: Stop for {seconds} seconds
Para Yellow: Caution for {seconds} seconds
Para Green: Go for {seconds} seconds
Tu salida debe mostrar el estado de cada semáforo en su propia línea:

Stop for {seconds} seconds
Caution for {seconds} seconds
Go for {seconds} seconds
Por ejemplo, con un semáforo en rojo de 30 segundos, amarillo de 5 segundos y verde de 25 segundos, la salida sería:

Stop for 30 seconds
Caution for 5 seconds
Go for 25 seconds
Recibirás tres entradas: la duración de la luz roja, la duración de la luz amarilla y la duración de la luz verde.
