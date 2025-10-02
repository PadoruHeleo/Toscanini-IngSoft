import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { invoke } from "@tauri-apps/api/core";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";

interface Props {
  resetKey?: number;
  onChange: (tipos: string[] | null) => void;
}

export function FiltrarEquiposPorTipo({ resetKey, onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [tiposDisponibles, setTiposDisponibles] = useState<string[]>([]);
  const [seleccionados, setSeleccionados] = useState<string[]>([]);
  const [busqueda, setBusqueda] = useState("");

  // Cargar tipos al abrir
  useEffect(() => {
    const fetchTipos = async () => {
      try {
        const lista = await invoke<string[]>("get_tipos_equipos");
        setTiposDisponibles(lista);
      } catch (err) {
        console.error("❌ Error cargando tipos:", err);
      }
    };
    fetchTipos();
  }, [resetKey]);

  // Reset automático cuando cambia resetKey
  useEffect(() => {
    setSeleccionados([]);
    setBusqueda("");
  }, [resetKey]);

  const toggleSeleccion = (tipo: string) => {
    setSeleccionados((prev) =>
      prev.includes(tipo) ? prev.filter((t) => t !== tipo) : [...prev, tipo]
    );
  };

  const limpiar = () => {
    setSeleccionados([]);
    setBusqueda("");
    onChange(null);
  };

  const aplicarFiltro = () => {
    onChange(seleccionados.length > 0 ? seleccionados : null);
    setOpen(false);
  };

  const tiposFiltrados = tiposDisponibles.filter((t) =>
    t.toLowerCase().includes(busqueda.toLowerCase())
  );

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por tipo
        {seleccionados.length > 0 && (
          <span className="ml-1 bg-blue-100 text-blue-800 px-1 rounded text-xs flex items-center gap-1">
            {seleccionados.length}
            <button
              onClick={(e) => {
                e.stopPropagation();
                limpiar();
              }}
              className="ml-1 text-red-500 hover:text-red-700 font-bold text-lg leading-none hover:bg-red-200 rounded-full px-1"
            >
              ×
            </button>
          </span>
        )}
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Seleccione que tipos quiere ver</DialogTitle>
          </DialogHeader>

          <Input
            placeholder="Buscar tipo..."
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
            className="mb-2"
          />

          <div className="max-h-64 overflow-y-auto space-y-2">
            {tiposFiltrados.map((tipo) => (
              <div key={tipo} className="flex items-center space-x-2">
                <Checkbox
                  checked={seleccionados.includes(tipo)}
                  onCheckedChange={() => toggleSeleccion(tipo)}
                />
                <span>{tipo}</span>
              </div>
            ))}
          </div>

          <DialogFooter className="flex gap-2">
            <Button onClick={aplicarFiltro}>
              Aplicar {seleccionados.length > 0 && `(${seleccionados.length})`}
            </Button>
            <Button variant="outline" onClick={limpiar}>
              Limpiar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
