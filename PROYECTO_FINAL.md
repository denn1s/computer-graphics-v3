# Proyecto final: un sistema de planetas diminutos

## Idea

Construirán un pequeño sistema solar que podrán explorar desde una nave. Los planetas pueden ser tan pequeños y particulares como los de *El principito*: uno puede tener una ciudad, otro un árbol enorme y otro una superficie rocosa. La nave podrá acercarse a ellos y rodearlos sin interrumpir la simulación ni cambiar de escena. **No se requiere aterrizar, caminar ni construir un mundo abierto.**

El objetivo principal es demostrar lo que aprendieron sobre el **renderizador por software**: cargar y transformar modelos, usar cámaras y profundidad, crear shaders y componer una escena animada que siga siendo interactiva.

Pueden trabajar individualmente o en grupos de dos o tres personas. El plazo de desarrollo es de aproximadamente cuatro semanas.

## Requisitos indispensables

**Si falta cualquiera de estos requisitos, el proyecto recibe 0 puntos.** Cumplirlos habilita la evaluación, pero la nota se obtiene con el menú de puntos de la sección siguiente.

1. Renderizar la escena tridimensional mediante su propio pipeline de software. Pueden usar raylib para crear la ventana, leer entradas y mostrar el framebuffer, pero no sus funciones de dibujo 3D, modelos, cámara 3D, materiales, shaders de GPU ni física. Pueden usar Blender para crear modelos y bibliotecas auxiliares para leer archivos, generar números aleatorios o calcular ruido; estas bibliotecas no deben sustituir las etapas evaluadas del renderizador. No pueden usar un motor de juego o de física.
2. Mostrar un sol, una nave modelada por ustedes y al menos un planeta. La nave debe poder desplazarse bajo el control de quien juega. El planeta debe orbitar el sol y rotar sobre su propio eje; los movimientos deben depender del tiempo transcurrido, no del número de cuadros dibujados.
3. Permitir que la nave se acerque lo suficiente al planeta para apreciar su superficie y luego se aleje, **sin pantalla de carga ni transición a otra escena**. No es necesario tocar la superficie. La cámara debe seguir permitiendo orientar la nave y ver el entorno durante el acercamiento.
4. Cargar y dibujar modelos tridimensionales mediante transformaciones de modelo, vista y proyección, con prueba de profundidad. Debe haber al menos un shader de fragmentos escrito por ustedes y aplicado a un objeto de la escena.
5. Entregar el código fuente, los modelos y demás recursos necesarios, instrucciones para ejecutar el proyecto y un video corto que muestre el recorrido completo: vuelo espacial, acercamiento, vuelta alrededor de un planeta y alejamiento.

## Menú de puntos

Pueden elegir qué implementar. Hay **200 puntos disponibles**, pero la nota máxima del proyecto es **100 puntos**: `nota = min(100, puntos obtenidos)`. Una característica suma puntos solo si funciona en la demostración y pueden explicar cómo está implementada. No se otorgan fracciones de los puntos indicados, excepto en los dos criterios de apreciación descritos abajo.

| Área | Característica | Puntos | Máximo |
| --- | --- | ---: | ---: |
| Planetas | Planeta distinguible que orbita, rota y puede observarse de cerca | 10 por planeta | 50 (5 planetas) |
| Superficie | Tipo de modelo propio colocado sobre un planeta, como árbol, roca, edificio o monumento | 5 por tipo | 20 (4 tipos) |
| Superficie | Distribución procedural reproducible de varias instancias sobre la superficie | 10 | 10 |
| Superficie | Posición y orientación correctas de objetos sobre la superficie curva mientras el planeta rota | 5 | 5 |
| Renderizado | Shader propio con un efecto visual claramente distinguible | 6 por shader | 24 (4 shaders) |
| Renderizado | Shader animado cuyo efecto cambia con el tiempo | 8 | 8 |
| Renderizado | Mapeo de normales implementado en el renderizador por software | 8 | 8 |
| Renderizado | Deformación de la geometría de un planeta mediante ruido u otra función | 10 | 10 |
| Renderizado | Etapa propia de generación o subdivisión procedural de geometría antes de rasterizar | 10 | 10 |
| Navegación | Modo de control local que facilita rodear un planeta cercano sin cortar el vuelo | 8 | 8 |
| Navegación | Aceleración, frenado y giro progresivos, con respuesta consistente entre equipos de distinta velocidad | 8 | 8 |
| Navegación | Atracción gravitatoria de los planetas que modifica la trayectoria de la nave | 10 | 10 |
| Navegación | Colisión básica que impide atravesar el planeta | 4 | 4 |
| Rendimiento | Descartar objetos fuera de la vista antes de rasterizarlos | 7 | 7 |
| Rendimiento | Usar distintas cantidades de detalle según la distancia a un planeta u objeto | 8 | 8 |
| Presentación | Coherencia visual y composición de la escena | hasta 5 | 5 |
| Presentación | Claridad de la experiencia: controles comprensibles y recorrido fácil de seguir | hasta 5 | 5 |
| | **Total disponible** | | **200** |

### Cómo se cuentan los puntos

- Cada planeta debe tener una identidad visible. Cambiar únicamente el color de una copia no cuenta como otro planeta distinguible. El primer planeta también puede sumar sus 10 puntos, aunque sea indispensable para habilitar la evaluación.
- Un tipo de modelo se cuenta una sola vez aunque se repita muchas veces. Las cuatro variaciones de escala de un mismo árbol siguen siendo un tipo. Los modelos de superficie deben ser creados o modificados sustancialmente por ustedes.
- Para la distribución procedural, usen una semilla fija o configurable: con la misma semilla deben obtener las mismas ubicaciones. Las instancias deben distribuirse sobre el planeta, no aparecer en posiciones escritas una por una.
- Cada shader puntuable debe producir un resultado visual diferente y tener una implementación diferente. Cambiar solo un color o una constante no crea un shader adicional. El shader indispensable puede sumar puntos si cumple este criterio. El shader animado puede ser uno de los shaders puntuados.
- La generación de geometría se realiza en el pipeline de software, antes de rasterizar; no se refiere a un geometry shader de GPU. La deformación de una esfera existente y la generación o subdivisión de nueva geometría son logros distintos.
- El descarte y los niveles de detalle deben cambiar el trabajo que hace el renderizador. Deberán mostrar una comparación o contadores simples de triángulos procesados para demostrarlo.
- En los dos criterios de presentación, se pueden otorgar de 0 a 5 puntos. Una dirección artística deliberada puede ser simple o poco convencional; no se exige realismo. Estos criterios suman **10 puntos en total**, por lo que también es posible obtener 100 puntos mediante logros técnicos.

## Entrega y demostración

Entreguen el repositorio con instrucciones claras de ejecución, controles, integrantes y contribuciones de cada persona. Incluyan una lista breve de las características del menú que solicitan evaluar y dónde pueden verse. Si usan recursos externos o asistencia de IA, indíquenlo; podrán usar ambas cosas, pero deberán comprender y explicar el código que entreguen.

Durante la demostración, cada integrante explicará una contribución técnica concreta. Podemos pedirles que cambien en vivo un parámetro relevante, por ejemplo el radio de un planeta, la velocidad orbital, la semilla de distribución o la intensidad de un shader, y que expliquen el resultado. La nota del proyecto se calcula con el menú; la explicación individual permite verificar la autoría y el aprendizaje de cada integrante.

## Alcance recomendado

Primero hagan que un planeta y una nave se vean y se muevan correctamente. Después logren un acercamiento estable. A partir de allí elijan una dirección: varios planetas, una superficie memorable, shaders más elaborados, controles más refinados o mejoras del renderizador. Un planeta pequeño con un rasgo central bien realizado puede ser tan interesante como un sistema grande.
