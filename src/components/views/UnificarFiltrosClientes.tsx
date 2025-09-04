import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { FiltrarOrdenesPorFechaClientes } from "./FiltrarOrdenesPorFechaClientes";
import { FiltrarClientesPorCorreo } from "./FiltrarClientesPorCorreo";
import { FiltrarClientesPorRuts } from "./FiltrarClientesPorRuts";
import { FiltrarClientesPorCiudad } from "./FiltrarClientesPorCiudad";

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

  // ✅ Sincronizar searchTerm con filtros de forma más eficiente
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

  // ✅ Función para actualizar filtros (sin afectar search)
  const actualizarFiltro = (nuevoFiltro: Partial<typeof filtros>) => {
    setFiltros((prev) => ({
      ...prev,
      ...nuevoFiltro,
    }));
  };

  // ✅ Aplicar filtros al backend
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
      onFiltrar(clientes);
    } catch (err) {
      console.error("❌ Error aplicando filtros:", err);
      // En caso de error, intentar cargar todos los clientes
      try {
        const clientesBackup = await invoke<Cliente[]>("get_clientes");
        onFiltrar(clientesBackup);
      } catch (backupErr) {
        console.error("❌ Error en carga de respaldo:", backupErr);
        onFiltrar([]);
      }
    }
  };

  // ✅ Aplicar filtros cuando cambien
  useEffect(() => {
    aplicarFiltros();
  }, [filtros]);

  // ✅ Verificar si hay filtros activos (incluyendo búsqueda)
  const hayFiltrosActivos =
    filtros.fecha_inicio !== null ||
    filtros.fecha_fin !== null ||
    filtros.correo !== null ||
    filtros.rut !== null ||
    filtros.ciudad !== null ||
    filtros.search !== null;

  // ✅ Limpiar todos los filtros (incluyendo búsqueda)
  const limpiarFiltros = () => {
    setFiltros(filtrosIniciales);
    setResetKey((prev) => prev + 1);

    // Opcional: También limpiar el input de búsqueda en el componente padre
    // Esto requeriría una prop adicional como onClearSearch
    // onClearSearch?.();
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

      {hayFiltrosActivos && (
        <Button variant="outline" onClick={limpiarFiltros} className="text-sm">
          Limpiar Filtros
        </Button>
      )}
    </div>
  );
}
