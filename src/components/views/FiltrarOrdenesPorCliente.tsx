import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { invoke } from "@tauri-apps/api/core";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";

interface Props {
  onChange: (clientes: string[]) => void;
  resetKey?: number; // 🔹 para resetear desde el padre
}

export function FiltrarOrdenesPorCliente({ onChange, resetKey }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionadas, setSeleccionadas] = useState<string[]>([]);
  const [clientesDisponibles, setClientesDisponibles] = useState<string[]>([]);
  const [clientesFiltrados, setClientesFiltrados] = useState<string[]>([]);
  const [textoBusqueda, setTextoBusqueda] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // 🔹 Limpia selección cuando cambia resetKey
  useEffect(() => {
    setSeleccionadas([]);
    onChange([]);
  }, [resetKey]);

  useEffect(() => {
    if (open) {
      cargarClientes();
    }
  }, [open]);

  useEffect(() => {
    if (textoBusqueda.trim() === "") {
      setClientesFiltrados(clientesDisponibles);
    } else {
      const filtrados = clientesDisponibles.filter((cliente) =>
        cliente.toLowerCase().includes(textoBusqueda.toLowerCase())
      );
      setClientesFiltrados(filtrados);
    }
  }, [textoBusqueda, clientesDisponibles]);

  const cargarClientes = async () => {
    setLoading(true);
    setError(null);
    try {
      const clientes = await invoke<string[]>("get_clientes_disponibles");
      setClientesDisponibles(clientes);
      setClientesFiltrados(clientes);
    } catch (err) {
      console.error("❌ Error cargando clientes:", err);
      setError("Error al cargar los clientes");
      const clientesEjemplo = [
        "Laboratorio ABC",
        "Clínica XYZ",
        "Hospital Central",
      ];
      setClientesDisponibles(clientesEjemplo);
      setClientesFiltrados(clientesEjemplo);
    } finally {
      setLoading(false);
    }
  };

  const toggleCliente = (cliente: string) => {
    setSeleccionadas((prev) =>
      prev.includes(cliente)
        ? prev.filter((x) => x !== cliente)
        : [...prev, cliente]
    );
  };

  const aplicar = () => {
    onChange(seleccionadas);
    setOpen(false);
  };

  const limpiar = () => {
    setSeleccionadas([]);
    onChange([]);
    setOpen(false);
  };

  const limpiarDirecto = (e: React.MouseEvent) => {
    e.stopPropagation();
    setSeleccionadas([]);
    onChange([]);
  };

  const handleOpenChange = (newOpen: boolean) => {
    setOpen(newOpen);
    if (!newOpen) {
      setTextoBusqueda("");
    }
  };

  return (
    <>
      <Button
        variant="outline"
        onClick={() => setOpen(true)}
        className="flex items-center"
      >
        Filtrar por Cliente
        {seleccionadas.length > 0 && (
          <span className="ml-1 bg-blue-100 text-blue-800 px-1 rounded text-xs flex items-center gap-1">
            {seleccionadas.length}
            <button
              onClick={limpiarDirecto}
              className="ml-1 text-red-500 hover:text-red-700 font-bold text-lg leading-none"
              title="Limpiar filtro"
            >
              ×
            </button>
          </span>
        )}
      </Button>

      <Dialog open={open} onOpenChange={handleOpenChange}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Selecciona Clientes</DialogTitle>
          </DialogHeader>

          <div className="space-y-3 mt-2">
            <div>
              <Input
                placeholder="Buscar cliente..."
                value={textoBusqueda}
                onChange={(e) => setTextoBusqueda(e.target.value)}
                disabled={loading}
                className="w-full"
              />
            </div>

            <div className="max-h-60 overflow-y-auto border rounded p-2">
              {loading ? (
                <div className="flex items-center justify-center py-4">
                  <span className="text-gray-500">Cargando clientes...</span>
                </div>
              ) : error ? (
                <div className="text-red-500 text-sm py-2">⚠️ {error}</div>
              ) : clientesFiltrados.length === 0 ? (
                <div className="text-gray-500 text-sm py-2">
                  {textoBusqueda.trim()
                    ? "No se encontraron clientes que coincidan"
                    : "No se encontraron clientes"}
                </div>
              ) : (
                <div className="space-y-1">
                  {clientesFiltrados.map((cliente) => (
                    <label
                      key={cliente}
                      className="flex items-center gap-2 hover:bg-gray-50 p-2 rounded cursor-pointer transition-colors"
                    >
                      <input
                        type="checkbox"
                        checked={seleccionadas.includes(cliente)}
                        onChange={() => toggleCliente(cliente)}
                        className="rounded"
                      />
                      <span className="flex-1 text-sm">{cliente}</span>
                    </label>
                  ))}
                </div>
              )}
            </div>

            {seleccionadas.length > 0 && (
              <div className="text-sm text-gray-600">
                <strong>Seleccionados:</strong> {seleccionadas.join(", ")}
              </div>
            )}
          </div>

          <DialogFooter className="gap-2">
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
