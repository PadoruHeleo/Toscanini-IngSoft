import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { FiltrarEquiposPorMarca } from "@/components/views/FiltrarEquiposPorMarca";
import { FiltrarEquiposPorTipo } from "@/components/views/FiltrarEquiposPorTipo";
import { FiltrarEquiposPorCliente } from "@/components/views/FiltrarEquiposPorCliente";
import { FiltrarEquiposPorEstado } from "@/components/views/FiltrarEquiposPorEstado";
import { FiltrarEquiposPorUbicacion } from "@/components/views/FiltrarEquiposPorUbicacion";

interface EquipoConEstado {
  equipo_id: number;
  numero_serie?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  equipo_tipo?: string;
  equipo_precio?: number;
  equipo_ubicacion?: string;
  cliente_id?: number;
  created_by?: number;
  created_at?: string;
  // Información del cliente
  cliente_nombre?: string;
  // Estado de la última orden de trabajo
  ultimo_estado_orden?: string;
  ultimo_codigo_orden?: string;
  fecha_ultima_orden?: string;
}

interface FiltrosEquipos {
  fecha_inicio?: string;
  fecha_fin?: string;
  marcas?: string[];
  modelos?: string[];
  tipos?: string[];
  clientes?: string[];
  ubicaciones?: string[];
  estados_orden?: string[];
  search?: string;
  ordenamiento?: string;
  precio_min?: number;
  precio_max?: number;
}

interface Props {
  searchTerm: string;
  onFiltrar: (equipos: EquipoConEstado[]) => void;
  onClearSearch: () => void;
}

export function UnificarFiltrosEquipos({
  searchTerm,
  onFiltrar,
  onClearSearch,
}: Props) {
  const [resetKey, setResetKey] = useState(0);
  const [filtros, setFiltros] = useState<FiltrosEquipos>({});
  const [loading, setLoading] = useState(false);

  const aplicarFiltros = async () => {
    try {
      setLoading(true);

      const filtrosCompletos: FiltrosEquipos = {
        ...filtros,
        search: searchTerm || undefined,
      };

      const equiposFiltrados = await invoke<EquipoConEstado[]>(
        "get_equipos_filtrados",
        { filtros: filtrosCompletos }
      );

      onFiltrar(equiposFiltrados);
    } catch (error) {
      console.error("Error aplicando filtros:", error);
    } finally {
      setLoading(false);
    }
  };

  // Aplicar filtros cuando cambian
  useEffect(() => {
    aplicarFiltros();
  }, [filtros, searchTerm]);

  const limpiarTodosFiltros = () => {
    setFiltros({});
    onClearSearch();
    setResetKey((prev) => prev + 1);
  };

  const hayFiltrosActivos = () => {
    return (
      searchTerm ||
      filtros.marcas?.length ||
      filtros.tipos?.length ||
      filtros.clientes?.length ||
      filtros.estados_orden?.length ||
      filtros.ubicaciones?.length
    );
  };

  return (
    <div className="flex flex-wrap gap-2 items-center">
      <FiltrarEquiposPorMarca
        resetKey={resetKey}
        onChange={(marcas: string[] | null) =>
          setFiltros((prev) => ({ ...prev, marcas: marcas || undefined }))
        }
      />

      <FiltrarEquiposPorTipo
        resetKey={resetKey}
        onChange={(tipos: string[] | null) =>
          setFiltros((prev) => ({ ...prev, tipos: tipos || undefined }))
        }
      />

      <FiltrarEquiposPorCliente
        resetKey={resetKey}
        onChange={(clientes: string[] | null) =>
          setFiltros((prev) => ({ ...prev, clientes: clientes || undefined }))
        }
      />

      <FiltrarEquiposPorEstado
        resetKey={resetKey}
        onChange={(estados_orden: string[] | null) =>
          setFiltros((prev) => ({
            ...prev,
            estados_orden: estados_orden || undefined,
          }))
        }
      />

      <FiltrarEquiposPorUbicacion
        resetKey={resetKey}
        onChange={(ubicaciones: string[] | null) =>
          setFiltros((prev) => ({
            ...prev,
            ubicaciones: ubicaciones || undefined,
          }))
        }
      />

      {hayFiltrosActivos() && (
        <Button variant="outline" onClick={limpiarTodosFiltros}>
          Limpiar todos los filtros
        </Button>
      )}

      {loading && <div className="text-sm text-gray-500">Filtrando...</div>}
    </div>
  );
}
