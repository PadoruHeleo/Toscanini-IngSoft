import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";

interface Props {
  onChange: (marcas: string[]) => void;
}

export function FiltrarOrdenesPorMarca({ onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionadas, setSeleccionadas] = useState<string[]>([]);
  const [marcasDisponibles, setMarcasDisponibles] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Cargar marcas cuando se abre el diálogo
  useEffect(() => {
    if (open) {
      cargarMarcas();
    }
  }, [open]);

  const cargarMarcas = async () => {
    setLoading(true);
    setError(null);

    try {
      console.log("🔄 Cargando marcas disponibles...");
      const marcas = await invoke<string[]>("get_marcas_disponibles");
      console.log("🏭 Marcas cargadas:", marcas);
      setMarcasDisponibles(marcas);
    } catch (err) {
      console.error("❌ Error cargando marcas:", err);
      setError("Error al cargar las marcas");
      // Fallback a datos de ejemplo si no se puede cargar desde BD
      setMarcasDisponibles(["Marca1", "Marca2", "Marca3"]);
    } finally {
      setLoading(false);
    }
  };

  const toggleMarca = (m: string) => {
    setSeleccionadas((prev) =>
      prev.includes(m) ? prev.filter((x) => x !== m) : [...prev, m]
    );
  };

  const aplicar = () => {
    console.log("🏭 Aplicando filtro de marcas:", seleccionadas);
    onChange(seleccionadas);
    setOpen(false);
  };

  const limpiar = () => {
    console.log("🧹 Limpiando filtro de marcas");
    setSeleccionadas([]);
    onChange([]);
    setOpen(false);
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Marca
        {seleccionadas.length > 0 && (
          <span className="ml-1 bg-blue-100 text-blue-800 px-1 rounded text-xs">
            {seleccionadas.length}
          </span>
        )}
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Selecciona Marcas</DialogTitle>
          </DialogHeader>

          <div className="space-y-2 mt-2 max-h-60 overflow-y-auto">
            {loading ? (
              <div className="flex items-center justify-center py-4">
                <span className="text-gray-500">Cargando marcas...</span>
              </div>
            ) : error ? (
              <div className="text-red-500 text-sm py-2">⚠️ {error}</div>
            ) : marcasDisponibles.length === 0 ? (
              <div className="text-gray-500 text-sm py-2">
                No se encontraron marcas
              </div>
            ) : (
              marcasDisponibles.map((m) => (
                <label
                  key={m}
                  className="flex items-center gap-2 hover:bg-gray-50 p-1 rounded cursor-pointer"
                >
                  <input
                    type="checkbox"
                    checked={seleccionadas.includes(m)}
                    onChange={() => toggleMarca(m)}
                    className="rounded"
                  />
                  <span>{m}</span>
                </label>
              ))
            )}
          </div>

          <DialogFooter>
            <Button onClick={aplicar} disabled={loading}>
              Aplicar {seleccionadas.length > 0 && `(${seleccionadas.length})`}
            </Button>
            <Button variant="outline" onClick={limpiar} disabled={loading}>
              Limpiar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
