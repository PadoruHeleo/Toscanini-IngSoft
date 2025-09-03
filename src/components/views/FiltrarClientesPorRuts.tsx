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
  onChange: (ruts: string[] | null) => void;
}

export function FiltrarClientesPorRuts({ resetKey, onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [rutsDisponibles, setRutsDisponibles] = useState<string[]>([]);
  const [seleccionados, setSeleccionados] = useState<string[]>([]);
  const [busqueda, setBusqueda] = useState("");

  // Cargar RUTs al abrir
  useEffect(() => {
    const fetchRuts = async () => {
      try {
        const lista = await invoke<string[]>("get_ruts_clientes");
        setRutsDisponibles(lista);
      } catch (err) {
        console.error("❌ Error cargando RUTs:", err);
      }
    };
    fetchRuts();
  }, [resetKey]);

  // Reset automático cuando cambia resetKey
  useEffect(() => {
    setSeleccionados([]);
    setBusqueda("");
  }, [resetKey]);

  // Función para manejar solo números en la búsqueda
  const handleBusquedaChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    // Solo permitir números y guiones (para formato RUT chileno)
    const numericValue = value.replace(/[^0-9-]/g, "");
    setBusqueda(numericValue);
  };

  const toggleSeleccion = (rut: string) => {
    setSeleccionados((prev) =>
      prev.includes(rut) ? prev.filter((r) => r !== rut) : [...prev, rut]
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

  const rutsFiltrados = rutsDisponibles.filter((r) =>
    r.toLowerCase().includes(busqueda.toLowerCase())
  );

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por RUT
        {seleccionados.length > 0 && (
          <span className="ml-1 bg-blue-100 text-blue-800 px-1 rounded text-xs flex items-center gap-1">
            {seleccionados.length}
            <button
              onClick={(e) => {
                e.stopPropagation(); // evita cerrar el diálogo
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
            <DialogTitle>Seleccione que RUTs quiere ver</DialogTitle>
          </DialogHeader>

          <Input
            placeholder="Buscar RUT (solo números)..."
            value={busqueda}
            onChange={handleBusquedaChange}
            className="mb-2"
            type="text"
            inputMode="numeric"
          />

          <div className="max-h-64 overflow-y-auto space-y-2">
            {rutsFiltrados.map((rut) => (
              <div key={rut} className="flex items-center space-x-2">
                <Checkbox
                  checked={seleccionados.includes(rut)}
                  onCheckedChange={() => toggleSeleccion(rut)}
                />
                <span>{rut}</span>
              </div>
            ))}
          </div>

          <DialogFooter>
            <Button onClick={aplicarFiltro}>
              Aplicar {seleccionados.length > 0 && `(${seleccionados.length})`}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
