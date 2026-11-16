Construyamos un sistema de notificaciones que use enumeraciones con datos para representar distintos tipos de alertas. Cada tipo de notificación contiene información diferente: algunas tienen mensajes, otras tienen valores numéricos y otras son señales simples.

Crearás dos archivos para organizar tu código:

notification.rs: Define una enumeración pública Notification con tres variantes:
Alert: contiene un único mensaje String (sintaxis de tupla)
Reminder: contiene campos con nombre: title (String) y minutes (u32)
Dismiss: no contiene datos, solo es una señal
main.rs: Incorpora tu módulo de notificaciones y crea una instancia de cada variante. Después, imprime información sobre cada notificación para demostrar que las tres variantes son del mismo tipo, pero contienen datos diferentes.
Al imprimir, usa el formato de depuración ({:?}) para mostrar cada notificación. Tendrás que derivar el trait Debug en tu enumeración para poder imprimirla de esta manera.

Tu salida debe mostrar las tres notificaciones, cada una en su propia línea:

Alert("{message}")
Reminder { title: "{title}", minutes: {minutes} }
Dismiss
Por ejemplo, con un mensaje de alerta "Server down", un recordatorio con el título "Meeting" y 30 minutos, la salida sería:

Alert("Server down")
Reminder { title: "Meeting", minutes: 30 }
Dismiss
Recibirás tres entradas: el mensaje de alerta, el título del recordatorio y los minutos del recordatorio.
