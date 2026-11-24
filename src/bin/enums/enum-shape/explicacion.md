Construyamos una calculadora de geometría que use un enum para representar diferentes figuras, cada una almacenando sus propias dimensiones. Tu enum Shape demostrará cómo un solo tipo puede contener diferentes estructuras de datos y proporcionar un comportamiento unificado mediante métodos.

Crearás dos archivos para organizar tu código:

shape.rs: Define un enum Shape público con dos variantes:
Circle: contiene un único f64 que representa el radio (sintaxis de tupla)
Rectangle: contiene campos con nombre: width (f64) y height (f64)
Implementa un método area que use coincidencia de patrones para calcular el área correcta según la variante. Para los círculos, usa 3.14159 como π.
main.rs: Importa tu módulo de figuras, crea una instancia de cada variante de figura usando las entradas proporcionadas e imprime el área de cada figura.
Tu método area debe devolver un f64. Al hacer la coincidencia, desestructura cada variante para extraer sus dimensiones y aplicar la fórmula correspondiente:

Área del círculo: π × radius²
Área del rectángulo: width × height
Tu salida debe mostrar el área de cada figura en su propia línea, con exactamente un decimal:

Circle area: {area}
Rectangle area: {area}
Por ejemplo, con un radio del círculo de 5.0 y un rectángulo con un ancho de 4.0 y una altura de 6.0, la salida sería:

Circle area: 78.5
Rectangle area: 24.0
Recibirás tres entradas: el radio del círculo, el ancho del rectángulo y la altura del rectángulo.
