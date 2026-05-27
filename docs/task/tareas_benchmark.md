# Listado de Tareas — Benchmarking Avanzado y Validación de Calidad Multidimensional

> **Fuentes de contexto obligatorias para el agente/desarrollador que ejecute estas tareas:**
> - Spec funcional/teórico: [Evaluación de Métricas de Conversión PDF a Markdown.md](../plan/Evaluación%20de%20Métricas%20de%20Conversión%20PDF%20a%20Markdown.md).
> - Plan maestro (Fases y arquitectura): [plan_benchmark.md](../plan/plan_benchmark.md).
> - Directorio de PDFs del corpus: `tests/fixtures/pdfs/articles` (201 artículos científicos).
>
> **Convenciones de checkbox**
> - `[ ]` Pendiente
> - `[/]` En progreso / en revisión
> - `[X]` Completado
> -
> **Convenciones de IDs**
> - **Fase:** `F{n}` — ej. `F1`
> - **Tarea:** `T{f}.{n}` — ej. `T1.2`
> - **Acción:** `A{f}.{t}.{n}` — ej. `A1.2.1`
> -
> **Regla maestra.** Cada **Acción** declara: **Objetivo · Input · Output · Proceso · Tests · Criterios de Aceptación (AC)**. Cada **Tarea** y **Fase** declara su propio Objetivo/AC global. Una Tarea solo se marca completada si todas sus Acciones están en `[X]` y sus AC se cumplen; una Fase solo se marca completada si todas sus Tareas lo están.

---

## Índice

- [Fase 0 — Setup y Dependencias de Métricas Avanzadas](#fase-0--setup-y-dependencias-de-métricas-avanzadas)
- [Fase 1 — Implementación del Motor de Métricas Científicas (quality_benchmark.py)](#fase-1--implementación-del-motor-de-métricas-científicas-quality_benchmarkpy)
- [Fase 2 — Orquestación y Pipelines Estadísticos (orchestrate_benchmarks.py)](#fase-2--orquestación-y-pipelines-estadísticos-orchestrate_benchmarkspy)
- [Fase 3 — Visualización Científica Multidimensional (plot_benchmark.py)](#fase-3--visualización-científica-multidimensional-plot_benchmarkpy)
- [Fase N — Integración, Testing y Verificación de Regresiones](#fase-n--integración-testing-y-verificación-de-regresiones)

---

## [X] Fase 0 — Setup y Dependencias de Métricas Avanzadas

- **Objetivo:** Preparar el entorno del proyecto agregando las dependencias científicas necesarias (`scipy`, `numpy`, `lxml`) de forma reproducible a través de `uv`, asegurando la paridad con el pipeline de CI/CD.
- **AC global de la Fase:**
  - El entorno virtual del proyecto se actualiza exitosamente con las nuevas librerías.
  - La suite de linters (`ruff`) y tipo estático (`mypy`) compilan de cero sin alertas.
- **Referencias:** [plan_benchmark.md - Fase 0](../plan/plan_benchmark.md#fase-0--setup-y-dependencias-de-métricas-avanzadas).

---

### [X] T0.1 — Configurar Dependencias del Entorno Científico

- **Objetivo:** Incorporar las librerías matemáticas y de análisis estructural necesarias para ejecutar los algoritmos del benchmark.
- **AC:** Ejecutar llamadas a los módulos integrados sin excepciones `ImportError`.

#### [X] A0.1.1 — Agregar scipy, numpy y lxml a pyproject.toml y uv.lock

- **Objetivo:** Declarar de forma explícita e instalar las dependencias científicas en el monorepo.
- **Input:** Gestor de paquetes `uv` instalado, archivo `pyproject.toml` base.
- **Output:** Modificaciones en `pyproject.toml` y sincronización de `uv.lock`.
- **Proceso:**
  1. Ejecutar en la raíz del repositorio:
     ```bash
     uv add scipy numpy lxml
     ```
  2. Verificar la correcta sincronización del lockfile.
- **Tests:**
  ```bash
  uv run python -c "import scipy, numpy, lxml; print('Dependencias listas!')"
  ```
- **AC:** El comando anterior retorna código de salida `0` e imprime "Dependencias listas!".

---

## [X] Fase 1 — Implementación del Motor de Métricas Científicas (`quality_benchmark.py`)

- **Objetivo:** Desarrollar e integrar las 4 dimensiones de métricas avanzadas (Hungarian matching para encabezados, TEDS para tablas, ANLS/JSD para texto continuo y IoU para figuras geométrica) en `quality_benchmark.py` bajo un enfoque determinista estricto.
- **AC global de la Fase:**
  - El archivo `quality_benchmark.py` cuenta con las definiciones matemáticas completas.
  - Pasa el análisis estático estricto de `mypy` en modo `strict`.
- **Referencias:** [plan_benchmark.md - Fase 1](../plan/plan_benchmark.md#fase-1--implementación-del-motor-de-métricas-científicas-quality_benchmarkpy).

---

### [X] T1.1 — Módulo de Asignación Óptima de Encabezados (Header Metric Engine)

- **Objetivo:** Resolver el apareamiento óptimo global entre títulos extraídos y el Ground Truth mediante la matriz de Levenshtein Normalizada y el Algoritmo Húngaro.
- **AC:** Discrepancias de nivel penalizan fuertemente la métrica `level_accuracy` de forma determinista.

#### [X] A1.1.1 — Extraer encabezados y construir matriz de costos de Levenshtein Normalizado

- **Objetivo:** Parsear los títulos en Markdown de la salida y de la referencia, y calcular sus distancias continuas.
- **Input:** Rutas a los Markdowns del convertidor y del Ground Truth (GT).
- **Output:** Función `compute_header_cost_matrix(predicted: list[str], target: list[str]) -> np.ndarray`.
- **Proceso:**
  - Buscar e identificar líneas que comiencen con `#` en ambos archivos.
  - Para cada par (predicho, GT), calcular la distancia Levenshtein Normalizada continuo:
    $$\text{Cost}(p, t) = \frac{\text{Levenshtein}(p, t)}{\max(\text{len}(p), \text{len}(t))}$$
- **Tests:** Probar con un subconjunto de títulos falsos y validar que la matriz de costos contenga dimensiones correctas e índices acotados entre `0.0` y `1.0`.
- **AC:** La matriz de costos tiene dimensiones $M \times N$ donde $M$ y $N$ son el conteo de encabezados de la predicción y de la referencia.

#### [X] A1.1.2 — Resolver asignación lineal con Algoritmo Húngaro y calcular Recall, Precision y Level Accuracy

- **Objetivo:** Resolver el emparejamiento bipartito de mínimo costo global y computar las precisiones.
- **Input:** Matriz de costos calculada en A1.1.1.
- **Output:** Función `evaluate_headers(pred_headers: list[str], gt_headers: list[str]) -> dict[str, float]`.
- **Proceso:**
  1. Invocar `scipy.optimize.linear_sum_assignment` sobre la matriz de costos.
  2. Descartar pares asignados cuya distancia sea mayor a `0.4`.
  3. Calcular `header_recall` y `header_precision`.
  4. Para los encabezados emparejados, verificar su nivel Markdown (número de `#`). Aplicar penalización asimétrica $0.0$ si el nivel difiere para calcular `level_accuracy`.
- **Tests:** Validar que una predicción con un encabezado extra de nivel incorrecto degrade el `level_accuracy` de forma proporcional.
- **AC:** Títulos aparejados con niveles de anidamiento dispares devuelven un `level_accuracy` inferior al 100% de manera consistente.

---

### [X] T1.2 — Módulo de Topología Tabular (TEDS Engine)

- **Objetivo:** Analizar y calcular la precisión estructural de tablas comparando su representación en árbol XML mediante la distancia de edición de árboles normalizada TEDS.
- **AC:** Faltas estructurales y de celdas combinadas de `colspan` o `rowspan` se penalizan severamente con costos máximos.

#### [X] A1.2.1 — Implementar parser de tablas XML/HTML con lxml

- **Objetivo:** Transformar representaciones de tablas de Markdown o HTML incrustado en árboles AST DOM navegables.
- **Input:** Bloques de código Markdown/HTML que delimiten tablas.
- **Output:** Función `parse_table_to_tree(table_str: str) -> lxml.etree._Element`.
- **Proceso:**
  - Extraer las tablas mediante expresiones regulares o etiquetas de control del Ground Truth y de las carpetas de salida.
  - Cargar el XML en memoria usando `lxml.html.fromstring` o `lxml.etree.fromstring`.
- **Tests:** Pasar fragmentos de tablas válidas e inválidas y verificar que el parser levante errores descriptivos solo para XMLs mal formados.
- **AC:** Retorna un nodo raíz compatible con `lxml` y sus respectivos nodos hijos (`tr`, `td`, `th`).

#### [X] A1.2.2 — Implementar algoritmo Zhang-Shasha de distancia de árboles tabulares y calcular TEDS

- **Objetivo:** Calcular el costo mínimo de transformación entre el árbol predicho y el del Ground Truth.
- **Input:** Dos árboles XML cargados por A1.2.1.
- **Output:** Función `compute_teds_score(pred_table: str, gt_table: str) -> float`.
- **Proceso:**
  - Implementar o abstraer el cálculo de distancia de edición de árboles.
  - Definir las reglas de penalización de costos:
    - Inserción/Eliminación de nodo = `1.0`.
    - Sustitución de nodo no compatible (ej. `tr` por `td`) = `1.0`.
    - Desviación geométrica de celdas (`colspan` o `rowspan` discrepante) = `1.0`.
    - Sustitución de texto en celdas coincidentes = Levenshtein normalizado de sus contenidos.
  - Aplicar la fórmula normalizada TEDS acotando el score final entre `0.0` y `1.0`.
- **Tests:** Validar que una celda predicha sin su correspondiente fusión `colspan="2"` devuelva un TEDS inferior a `0.85` de forma determinista.
- **AC:** Tablas sintácticas idénticas devuelven un TEDS exacto de `1.0`; tablas con dislocación de columnas devuelven scores severamente castigados.

---

### [X] T1.3 — Módulo de Coherencia de Flujo (ANLS & JSD)

- **Objetivo:** Evaluar cuantitativamente la precisión de la extracción lineal del cuerpo de texto e identificar ruidos de paginación o repetición.
- **AC:** Errores de ligaduras tipográficas se asimilan suavemente, pero inserciones redundantes disparan la divergencia JSD de forma proporcional.

#### [X] A1.3.1 — Programar similitud de tokens ANLS

- **Objetivo:** Medir la similitud de palabras normalizada tolerando ligeros fallos visuales.
- **Input:** Texto plano continuo.
- **Output:** Función `compute_anls(predicted_text: str, gt_text: str) -> float`.
- **Proceso:**
  - Tokenizar ambos textos a nivel de unigramas (palabras).
  - Alinear las secuencias y calcular para cada par la similitud de Levenshtein Normalizada:
    $$\text{NLS}(p, t) = 1.0 - \frac{\text{Levenshtein}(p, t)}{\max(\text{len}(p), \text{len}(t))}$$
  - Retornar la media aritmética acumulada.
- **Tests:** Pasar textos idénticos con una ligadura rota (ej. "fi" vs "f i") y corroborar que devuelva un score superior a `0.95`.
- **AC:** Pequeños fallos ópticos no penalizan binariamente la precisión de la oración.

#### [X] A1.3.2 — Programar divergencia probabilística JSD sobre frecuencias léxicas

- **Objetivo:** Detectar la inclusión repetitiva de marcas de agua o encabezados flotantes en cada página.
- **Input:** Diccionarios de frecuencias de palabras.
- **Output:** Función `compute_jensen_shannon_divergence(pred_text: str, gt_text: str) -> float`.
- **Proceso:**
  - Construir distribuciones de probabilidad discreta de frecuencia de vocabulario para ambos documentos.
  - Calcular la divergencia JSD basada en entropía mutua y divergencia Kullback-Leibler.
- **Tests:** Inyectar artificialmente la cabecera "arXiv:..." diez veces en el texto predicho y verificar que la JSD sobrepase el límite empírico de `0.15`.
- **AC:** Textos con altos niveles de ruido repetitivo devuelven divergencias JSD elevadas.

---

### [X] T1.4 — Módulo de Superposición Geométrica de Figuras (IoU Figure Engine)

- **Objetivo:** Certificar la correspondencia espacial y fidelidad de los recortes de imágenes y gráficos vectoriales.
- **AC:** Coordenadas desplazadas reducen de forma continua la métrica de superposición IoU.

#### [X] A1.4.1 — Extraer coordenadas de figuras y calcular IoU

- **Objetivo:** Parsear las cajas delimitadoras de las etiquetas HTML `<figure>` y computar su superposición física.
- **Input:** Etiquetas de figura extendidas en el Ground Truth y la salida.
- **Output:** Función `compute_iou(box1: list[float], box2: list[float]) -> float`.
- **Proceso:**
  - Extraer los vectores bounding box `[x0, y0, x1, y1]` de la etiqueta `bbox="..."`.
  - Calcular el área de intersección y el área de unión de ambos polígonos.
  - Dividir intersección entre unión.
- **Tests:** Pasar dos cajas de $100 \times 100$ desplazadas en $20$ píxeles y validar que devuelva un IoU cercano al $0.66$.
- **AC:** Cajas disjuntas devuelven un IoU exacto de `0.0`; cajas idénticas devuelven un `1.0`.

#### [X] A1.4.2 — Alinear figuras concurrentes con Algoritmo Húngaro y calcular Localization Accuracy

- **Objetivo:** Aparear múltiples figuras presentes en una misma página y verificar su orden jerárquico.
- **Input:** Listado de figuras extraídas del GT y de la predicción.
- **Output:** Función `evaluate_figures(pred_figs: list[dict], gt_figs: list[dict]) -> dict[str, float]`.
- **Proceso:**
  1. Construir la matriz de costo geométrico invertido ($1.0 - \text{IoU}$).
  2. Resolver el emparejamiento óptimo con `scipy.optimize.linear_sum_assignment`.
  3. Validar si la etiqueta del Markdown predicho está ubicada bajo la jerarquía del encabezado correcto en el documento de salida.
- **Tests:** Intercambiar de lugar dos figuras en la predicción y verificar que el algoritmo húngaro asigne correctamente las parejas y castigue la precisión de localización.
- **AC:** Figuras mal posicionadas jerárquicamente degradan el `localization_accuracy` de forma medible.

---

## [X] Fase 2 — Orquestación y Pipelines Estadísticos (`orchestrate_benchmarks.py`)

- **Objetivo:** Adaptar el script director de orquesta maestro para procesar secuencialmente el corpus masivo de 201 PDFs, mapeándolos con sus respectivos Ground Truths y escribiendo un reporte estadístico en JSON libre de suposiciones.
- **AC global de la Fase:**
  - El script `orchestrate_benchmarks.py` se ejecuta completo de inicio a fin sin colapsos por archivos faltantes.
  - Genera el archivo consolidado `strata_real_metrics.json` conteniendo todas las nuevas métricas estadísticas avanzadas.
- **Referencias:** [plan_benchmark.md - Fase 2](../plan/plan_benchmark.md#fase-2--orquestación-y-pipelines-estadísticos-orchestrate_benchmarkspy).

---

### [X] T2.1 — Enrutamiento Dinámico de Fixtures y Alineación de Archivos

- **Objetivo:** Implementar la carga resiliente de archivos del corpus y asociarlos contra sus homólogos canónicos del Ground Truth.
- **AC:** Archivos PDF sin anotación en el GT son omitidos graciosamente imprimiendo una advertencia, en lugar de romper el pipeline de 201 PDFs.

#### [X] A2.1.1 — Implementar emparejamiento resiliente de PDFs y Ground Truths de 201 artículos

- **Objetivo:** Emparejar de forma segura cada salida de convertidor con su archivo de referencia.
- **Input:** Directorio de artículos PDFs y carpeta `tests/fixtures/salidas/ground_truth`.
- **Output:** Mapeo de archivos comunes alineados.
- **Proceso:**
  - Iterar sobre los PDFs descubiertos en `tests/fixtures/pdfs/articles`.
  - Buscar un archivo `.md` de idéntico nombre en la carpeta del Ground Truth.
  - Si no existe, imprimir una advertencia detallando el archivo ausente y continuar con el siguiente.
- **Tests:** Mover temporalmente un archivo del GT fuera de la carpeta y correr la orquestación para validar que imprima la advertencia y complete el benchmark con los 200 restantes sin lanzar excepciones.
- **AC:** El pipeline finaliza con éxito procesando el subconjunto disponible y reportando de forma elegante la cobertura de archivos anotados.

---

### [X] T2.2 — Consolidación de Métricas en Matriz JSON

- **Objetivo:** Integrar el cálculo de las 4 nuevas métricas analíticas e integrar las métricas de rendimiento y velocidad de CPU en el archivo consolidado real.
- **AC:** Generación de un JSON plano ordenado y formateado listo para el graficador.

#### [X] A2.2.1 — Integrar llamadas a las 4 métricas avanzadas en el bucle principal y escribir strata_real_metrics.json

- **Objetivo:** Unir el cálculo de velocidades por página con el motor de `quality_benchmark.py` y escribir la salida estructurada.
- **Input:** Mediciones de tiempos y las precisiones obtenidas en la Fase 1.
- **Output:** Archivo `tests/fixtures/salidas/strata_real_metrics.json`.
- **Proceso:**
  - En cada paso del orquestador, recolectar velocidad del motor, páginas y procesar el Markdown resultante contra el GT usando `quality_benchmark.py`.
  - Consolidar las precisiones promedio globales para Strata-Reader, OpenDataLoader y Microsoft MarkItDown.
  - Serializar el diccionario resultante con indentación 2 a `strata_real_metrics.json`.
- **Tests:** Inspeccionar la estructura del JSON generado para validar que no posea campos vacíos u nulos.
- **AC:** El JSON resultante posee llaves estructuradas como `strata_teds`, `opendataloader_teds`, `strata_anls`, `opendataloader_anls`, etc., con datos numéricos reales.

---

## [X] Fase 3 — Visualización Científica Multidimensional (`plot_benchmark.py`)

- **Objetivo:** Refactorizar la generación visual para dibujar un reporte premium libre de sesgos en escalas de barras o superposición de textos.
- **AC global de la Fase:**
  - El script `plot_benchmark.py` dibuja y escribe la imagen de salida.
  - El gráfico `benchmark_comparison.png` se renderiza de forma limpia y legible.
- **Referencias:** [plan_benchmark.md - Fase 3](../plan/plan_benchmark.md#fase-3--visualización-científica-multidimensional-plot_benchmarkpy).

---

### [X] T3.1 — Generación de Gráfico Multivariable Comparativo

- **Objetivo:** Visualizar gráficamente las dimensiones analíticas (Velocidad, TEDS, ANLS, IoU, Encabezados) por motor sin mentiras visuales.
- **AC:** Generación del PNG premium sin colisiones de texto ni distorsión en la escala de barras.

#### [X] A3.1.1 — Refactorizar plot_benchmark.py para visualizar TEDS, ANLS, IoU y Header Accuracy sin distorsiones

- **Objetivo:** Dibujar los resultados consolidados de manera paralela y multivariable.
- **Input:** Archivo `strata_real_metrics.json`.
- **Output:** Imagen premium `tests/fixtures/salidas/benchmark_comparison.png`.
- **Proceso:**
  1. Leer el JSON consolidado de forma estricta (lanzar excepción si no existe o faltan llaves).
  2. Inicializar una figura de barras agrupadas o un mapa radial en `matplotlib`.
  3. Aplicar colores premium de marca (Azul Strata, Gris ODL, Fucsia Microsoft).
  4. Guardar en formato PNG premium.
- **Tests:** Verificar visualmente la imagen para comprobar que los ejes y etiquetas sean legibles.
- **AC:** La imagen se genera exitosamente y se guarda en la carpeta de salidas con un diseño wow y premium.

---

## [X] Fase N — Integración, Testing y Verificación de Regresiones

- **Objetivo:** Implementar la suite de pruebas unitarias sobre las funciones de métricas avanzadas para garantizar aserciones deterministas fiables.
- **AC global de la Fase:**
  - La suite de pruebas de Python se integra exitosamente en el framework del monorepo.
  - La validación aprueba el 100% de los tests unitarios creados.
- **Referencias:** [plan_benchmark.md - Fase N](../plan/plan_benchmark.md#fase-n--integración-testing-y-verificación-de-regresiones).

---

### [X] TN.1 — Suite de Pruebas Unitarias para el Motor de Métricas

- **Objetivo:** Blindar la lógica matemática ante regresiones accidentales de código.
- **AC:** Ejecución limpia de `pytest` retornando éxito.

#### [X] A1.N.1 — Crear test_advanced_metrics.py con aserciones estrictas para casos sintéticos

- **Objetivo:** Escribir y aislar la lógica de pruebas de las métricas en un archivo dedicado.
- **Input:** Módulos de `quality_benchmark.py`.
- **Output:** Archivo de pruebas `tests/unit_py/test_advanced_metrics.py`.
- **Proceso:**
  - Implementar funciones de test con decoradores `@pytest.mark.fixtures`.
  - Alimentar con casos sintéticos diseñados a mano (ej: comparar una tabla GT con una predicción a la que le falte un tag structural y asertar que el TEDS score sea menor a `0.85`).
- **Tests:**
  ```bash
  uv run pytest tests/unit_py/test_advanced_metrics.py
  ```
- **AC:** El comando de pruebas finaliza exitosamente con código `0` y aprueba todas las aserciones.
