import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { invoke } from "@tauri-apps/api/core";

interface OrdenTrabajo {
  orden_id: number;
  orden_codigo?: string;
  orden_desc?: string;
  prioridad?: string;
  estado?: string;
  has_garantia?: boolean;
  created_at?: string;
}

interface Props {
  onFiltrar: (ordenes: OrdenTrabajo[]) => void;
}

export function FiltrarOrdenesPorModelo({ onFiltrar }: Props) {
  const [modelos, setModelos] = useState<string[]>([]);
  const [modelosSeleccionados, setModelosSeleccionados] = useState<string[]>(
    []
  );
  const [open, setOpen] = useState(false);

  // Cargar modelos cuando se abre el modal
  useEffect(() => {
    if (!open) return;
    (async () => {
      try {
        const data = await invoke<string[]>("get_modelos_equipos");
        setModelos(data);
      } catch (err) {
        console.error("Error cargando modelos:", err);
        setModelos([]);
      }
    })();
  }, [open]);

  const toggleModelo = (modelo: string) => {
    setModelosSeleccionados((prev) =>
      prev.includes(modelo)
        ? prev.filter((m) => m !== modelo)
        : [...prev, modelo]
    );
  };

  const aplicarFiltro = async () => {
    if (modelosSeleccionados.length === 0) return;
    try {
      const ordenes = await invoke<OrdenTrabajo[]>(
        "get_ordenes_trabajo_por_modelos",
        { modelos: modelosSeleccionados }
      );
      onFiltrar(ordenes);
      setOpen(false);
    } catch (err) {
      console.error("Error filtrando por modelos:", err);
    }
  };

  const quitarFiltro = async () => {
    try {
      const ordenes = await invoke<OrdenTrabajo[]>("get_ordenes_trabajo");
      onFiltrar(ordenes);
      setModelosSeleccionados([]);
      setOpen(false);
    } catch (err) {
      console.error("Error quitando filtro:", err);
    }
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Modelo
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle>Filtrar por Modelo</DialogTitle>
          </DialogHeader>

          <div className="space-y-3 mt-2 max-h-72 overflow-auto pr-2">
            {modelos.length === 0 ? (
              <p className="text-sm text-gray-500">
                No hay modelos registrados.
              </p>
            ) : (
              <div className="space-y-2">
                {modelos.map((m) => (
                  <label key={m} className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      value={m}
                      checked={modelosSeleccionados.includes(m)}
                      onChange={() => toggleModelo(m)}
                    />
                    <span className="truncate">{m}</span>
                  </label>
                ))}
              </div>
            )}
          </div>

          <DialogFooter className="mt-4">
            <Button
              onClick={aplicarFiltro}
              disabled={modelosSeleccionados.length === 0}
            >
              Aplicar
            </Button>
            <Button variant="outline" onClick={quitarFiltro}>
              Quitar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
