import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { FiltrarOrdenesPorFechaClientes } from "./FiltrarOrdenesPorFechaClientes";
import { FiltrarClientesPorCorreo } from "./FiltrarClientesPorCorreo";
import { FiltrarClientesPorRuts } from "./FiltrarClientesPorRuts";
import { FiltrarClientesPorCiudad } from "./FiltrarClientesPorCiudad";
import { ArrowUpDown, ArrowUp, ArrowDown } from "lucide-react";

interface Cliente {
  cliente_id: number;
  cliente_rut?: string;
  cliente_nombre?: string;
  cliente_correo?: string;
  cliente_telefono?: string;
  cliente_direccion?: string;
  created_by?: number;
  created_at?: string;
}

type OrdenTipo = "ninguno" | "asc" | "desc";

interface Props {
  onFiltrar: (clientes: Cliente[]) => void;
  searchTerm?: string;
}

export function UnificarFiltrosClientes({ onFiltrar, searchTerm }: Props) {
  const filtrosIniciales = {
    fecha_inicio: null as string | null,
    fecha_fin: null as string | null,
    correo: null as string[] | null,
    rut: null as string[] | null,
    ciudad: null as string[] | null,
    search: null as string | null,
  };

  const [filtros, setFiltros] = useState(filtrosIniciales);
  const [resetKey, setResetKey] = useState(0);
  const [ordenamiento, setOrdenamiento] = useState<OrdenTipo>("ninguno");
  const [clientesOriginales, setClientesOriginales] = useState<Cliente[]>([]);

  //  Sincronizar searchTerm con filtros de forma más eficiente
  useEffect(() => {
    const searchValue = searchTerm?.trim() || null;

    // Solo actualizar si realmente cambió para evitar re-renders innecesarios
    setFiltros((prev) => {
      if (prev.search !== searchValue) {
        return { ...prev, search: searchValue };
      }
      return prev;
    });
  }, [searchTerm]);

  //  Función para actualizar filtros (sin afectar search)
  const actualizarFiltro = (nuevoFiltro: Partial<typeof filtros>) => {
    setFiltros((prev) => ({
      ...prev,
      ...nuevoFiltro,
    }));
  };

  //  Función para ordenar clientes alfabéticamente por nombre
  const ordenarClientes = (clientes: Cliente[], tipo: OrdenTipo): Cliente[] => {
    if (tipo === "ninguno") return clientes;

    return [...clientes].sort((a, b) => {
      const nombreA = (a.cliente_nombre || "").toLowerCase();
      const nombreB = (b.cliente_nombre || "").toLowerCase();

      if (tipo === "asc") {
        return nombreA.localeCompare(nombreB);
      } else {
        return nombreB.localeCompare(nombreA);
      }
    });
  };

  //  Manejar cambio de ordenamiento (solo cicla entre asc y desc)
  const manejarOrdenamiento = () => {
    let nuevoOrden: OrdenTipo;

    // Si no hay ordenamiento activo, empezar con A-Z
    if (ordenamiento === "ninguno") {
      nuevoOrden = "asc";
    }
    // Si está en A-Z, cambiar a Z-A
    else if (ordenamiento === "asc") {
      nuevoOrden = "desc";
    }
    // Si está en Z-A, cambiar a A-Z
    else {
      nuevoOrden = "asc";
    }

    setOrdenamiento(nuevoOrden);

    // Aplicar ordenamiento a los clientes actuales
    const clientesOrdenados = ordenarClientes(clientesOriginales, nuevoOrden);
    onFiltrar(clientesOrdenados);
  };

  // Aplicar filtros al backend
  const aplicarFiltros = async () => {
    try {
      console.log("🔍 Aplicando filtros de clientes:", filtros);

      // Si no hay ningún filtro activo, obtener todos los clientes
      const hayFiltrosActivos =
        filtros.fecha_inicio !== null ||
        filtros.fecha_fin !== null ||
        filtros.correo !== null ||
        filtros.rut !== null ||
        filtros.ciudad !== null ||
        filtros.search !== null;

      let clientes: Cliente[];

      if (!hayFiltrosActivos) {
        // Sin filtros, obtener todos los clientes
        clientes = await invoke<Cliente[]>("get_clientes");
      } else {
        // Con filtros, usar el endpoint de filtrado
        const filtrosParaBackend = {
          fecha_inicio: filtros.fecha_inicio,
          fecha_fin: filtros.fecha_fin,
          correo: filtros.correo,
          rut: filtros.rut,
          ciudad: filtros.ciudad,
          search: filtros.search,
        };

        console.log("📤 Enviando al backend:", filtrosParaBackend);
        clientes = await invoke<Cliente[]>("get_clientes_filtrados", {
          filtros: filtrosParaBackend,
        });
      }

      console.log("📨 Clientes recibidos:", clientes.length);

      // Guardar los clientes originales (sin ordenar)
      setClientesOriginales(clientes);

      // Aplicar ordenamiento si está activo
      const clientesOrdenados = ordenarClientes(clientes, ordenamiento);
      onFiltrar(clientesOrdenados);
    } catch (err) {
      console.error("❌ Error aplicando filtros:", err);
      // En caso de error, intentar cargar todos los clientes
      try {
        const clientesBackup = await invoke<Cliente[]>("get_clientes");
        setClientesOriginales(clientesBackup);
        const clientesOrdenados = ordenarClientes(clientesBackup, ordenamiento);
        onFiltrar(clientesOrdenados);
      } catch (backupErr) {
        console.error("❌ Error en carga de respaldo:", backupErr);
        setClientesOriginales([]);
        onFiltrar([]);
      }
    }
  };

  //  Aplicar filtros cuando cambien
  useEffect(() => {
    aplicarFiltros();
  }, [filtros]);

  //  Verificar si hay filtros activos (incluyendo búsqueda)
  const hayFiltrosActivos =
    filtros.fecha_inicio !== null ||
    filtros.fecha_fin !== null ||
    filtros.correo !== null ||
    filtros.rut !== null ||
    filtros.ciudad !== null ||
    filtros.search !== null;

  //  Limpiar todos los filtros (incluyendo búsqueda y ordenamiento)
  const limpiarFiltros = () => {
    setFiltros(filtrosIniciales);
    setOrdenamiento("ninguno");
    setResetKey((prev) => prev + 1);
  };

  //  Función para obtener el icono del botón de ordenamiento
  const obtenerIconoOrdenamiento = () => {
    switch (ordenamiento) {
      case "asc":
        return <ArrowUp className="h-4 w-4" />;
      case "desc":
        return <ArrowDown className="h-4 w-4" />;
      default:
        return <ArrowUpDown className="h-4 w-4" />;
    }
  };

  //  Función para obtener el texto del botón de ordenamiento
  const obtenerTextoOrdenamiento = () => {
    switch (ordenamiento) {
      case "asc":
        return "A-Z";
      case "desc":
        return "Z-A";
      default:
        return "Ordenar";
    }
  };

  return (
    <div className="flex gap-2 flex-wrap items-center">
      <FiltrarOrdenesPorFechaClientes
        resetKey={resetKey}
        onChange={({ fechaInicio, fechaFin }) => {
          actualizarFiltro({ fecha_inicio: fechaInicio, fecha_fin: fechaFin });
        }}
      />

      <FiltrarClientesPorCorreo
        resetKey={resetKey}
        onChange={(correos) => actualizarFiltro({ correo: correos })}
      />

      <FiltrarClientesPorRuts
        resetKey={resetKey}
        onChange={(ruts) => actualizarFiltro({ rut: ruts })}
      />

      <FiltrarClientesPorCiudad
        resetKey={resetKey}
        onChange={(ciudades) => actualizarFiltro({ ciudad: ciudades })}
      />

      {/* Botón de ordenamiento alfabético */}
      <Button
        variant={ordenamiento !== "ninguno" ? "default" : "outline"}
        onClick={manejarOrdenamiento}
        className="text-sm flex items-center gap-1"
        title={`Ordenamiento actual: ${
          ordenamiento === "ninguno"
            ? "Sin ordenar"
            : ordenamiento === "asc"
            ? "Ascendente (A-Z)"
            : "Descendente (Z-A)"
        }`}
      >
        {obtenerIconoOrdenamiento()}
        {obtenerTextoOrdenamiento()}
      </Button>

      {(hayFiltrosActivos || ordenamiento !== "ninguno") && (
        <Button variant="outline" onClick={limpiarFiltros} className="text-sm">
          Limpiar Todo
        </Button>
      )}
    </div>
  );
}
