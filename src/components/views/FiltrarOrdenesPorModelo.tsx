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
  onChange: (modelos: string[]) => void;
}

export function FiltrarOrdenesPorModelo({ onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionadas, setSeleccionadas] = useState<string[]>([]);
  const [modelosDisponibles, setModelosDisponibles] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Cargar modelos cuando se abre el diálogo
  useEffect(() => {
    if (open) {
      cargarModelos();
    }
  }, [open]);

  const cargarModelos = async () => {
    setLoading(true);
    setError(null);

    try {
      console.log("🔄 Cargando modelos disponibles...");
      const modelos = await invoke<string[]>("get_modelos_disponibles");
      console.log("📱 Modelos cargados:", modelos);
      setModelosDisponibles(modelos);
    } catch (err) {
      console.error("❌ Error cargando modelos:", err);
      setError("Error al cargar los modelos");
      // Fallback a datos de ejemplo si no se puede cargar desde BD
      setModelosDisponibles(["Modelo1", "Modelo2", "Modelo3"]);
    } finally {
      setLoading(false);
    }
  };

  const toggleModelo = (m: string) => {
    setSeleccionadas((prev) =>
      prev.includes(m) ? prev.filter((x) => x !== m) : [...prev, m]
    );
  };

  const aplicar = () => {
    console.log("📱 Aplicando filtro de modelos:", seleccionadas);
    onChange(seleccionadas);
    setOpen(false);
  };

  const limpiar = () => {
    console.log("🧹 Limpiando filtro de modelos");
    setSeleccionadas([]);
    onChange([]);
    setOpen(false);
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Modelo
        {seleccionadas.length > 0 && (
          <span className="ml-1 bg-blue-100 text-blue-800 px-1 rounded text-xs">
            {seleccionadas.length}
          </span>
        )}
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Selecciona Modelos</DialogTitle>
          </DialogHeader>

          <div className="space-y-2 mt-2 max-h-60 overflow-y-auto">
            {loading ? (
              <div className="flex items-center justify-center py-4">
                <span className="text-gray-500">Cargando modelos...</span>
              </div>
            ) : error ? (
              <div className="text-red-500 text-sm py-2">⚠️ {error}</div>
            ) : modelosDisponibles.length === 0 ? (
              <div className="text-gray-500 text-sm py-2">
                No se encontraron modelos
              </div>
            ) : (
              modelosDisponibles.map((m) => (
                <label
                  key={m}
                  className="flex items-center gap-2 hover:bg-gray-50 p-1 rounded cursor-pointer"
                >
                  <input
                    type="checkbox"
                    checked={seleccionadas.includes(m)}
                    onChange={() => toggleModelo(m)}
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
