Construyamos un sistema de sensor de temperatura que demuestre la encapsulación manteniendo los datos sensibles protegidos dentro de un módulo.

Crearás dos archivos para organizar tu código:

sensor.rs: Define una estructura TemperatureSensor con dos campos: location (String) y reading (f64). La estructura debe ser pública para que pueda usarse desde otros archivos, pero mantén ambos campos privados: el código externo no debería poder acceder directamente a los datos del sensor ni modificarlos. Añade un constructor público new que reciba un nombre de ubicación y una lectura de temperatura inicial.
main.rs: Incluye tu módulo de sensor y crea un TemperatureSensor usando el constructor. Como los campos son privados, no podrás imprimirlos directamente; en su lugar, imprime un mensaje de confirmación que muestre que el sensor se creó correctamente.
Este desafío se centra en el concepto fundamental de los campos privados: se puede acceder a la estructura, pero sus datos internos están protegidos. El código externo solo puede interactuar con el sensor mediante los métodos que proporciones (en este caso, únicamente el constructor).

Tu salida debe seguir exactamente este formato:

Sensor created for: {location}
Para acceder a la ubicación al imprimirla, tendrás que hacer que tu función new imprima este mensaje antes de devolver el sensor, ya que el campo en sí es privado.

Por ejemplo, si creas un sensor con la ubicación "Kitchen" y la lectura 22.5, la salida sería:

Sensor created for: Kitchen
Recibirás dos entradas: el nombre de la ubicación y la lectura de temperatura inicial.
