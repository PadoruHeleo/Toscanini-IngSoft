import { useEffect, useState } from "react";
import { useToastContext } from "@/contexts/ToastContext";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";

type TipoAccesorio = {
  tipo_id: number;
  nombre?: string;
  created_at?: string;
};

type OrdenAccesorio = {
  id?: number;
  orden_id?: number;
  tipo_accesorio_id: number;
  estado?: string;
  observaciones?: string | null;
  created_at?: string;
};

export default function AccesoriosForm({
  ordenId,
  value,
  onChange,
}: {
  ordenId?: number | null;
  value?: OrdenAccesorio[];
  onChange?: (v: OrdenAccesorio[]) => void;
}) {
  const [tipos, setTipos] = useState<TipoAccesorio[]>([]);
  const [selected, setSelected] = useState<OrdenAccesorio[]>(value || []);
  const [newTipoName, setNewTipoName] = useState("");
  const [creating, setCreating] = useState(false);
  const { success, error: showError } = useToastContext();

  useEffect(() => {
    loadTipos();
  }, []);

  useEffect(() => {
    setSelected(value || []);
  }, [value]);

  async function loadTipos(): Promise<TipoAccesorio[]> {
    try {
      const res = await invoke<TipoAccesorio[]>("get_tipos_accesorios");
      const list = res || [];
      setTipos(list);
      return list;
    } catch (e) {
      console.error("Error cargando tipos de accesorios", e);
      setTipos([]);
      return [];
    }
  }

  function addTipoToSelected(tipoId: number) {
    if (selected.find((s) => s.tipo_accesorio_id === tipoId)) return;
    const next = [
      ...selected,
      { tipo_accesorio_id: tipoId, estado: "presente" },
    ];
    setSelected(next);
    onChange?.(next);
  }

  function removeTipoFromSelected(tipoId: number) {
    const next = selected.filter((s) => s.tipo_accesorio_id !== tipoId);
    setSelected(next);
    onChange?.(next);
  }

  async function handleCreateTipo() {
    if (!newTipoName.trim()) return;
    try {
      setCreating(true);
      const nameToSend = newTipoName.trim();
      const created = await invoke<any>("create_tipo_accesorio", {
        request: { nombre: nameToSend },
      });
      console.debug("create_tipo_accesorio response:", created);
      // reload tipos and attempt to find created type
      const tiposRes = await loadTipos();
      setNewTipoName("");
      if (created && created.tipo_id) {
        addTipoToSelected(created.tipo_id);
        success(
          "Tipo creado",
          `Se ha creado el tipo ${created.nombre || nameToSend}`
        );
      } else {
        // Fallback: buscar por nombre (case-insensitive)
        const match = tiposRes.find(
          (t) =>
            (t.nombre || "").toLowerCase() === newTipoName.trim().toLowerCase()
        );
        if (match) {
          addTipoToSelected(match.tipo_id);
          success(
            "Tipo creado",
            `Se ha creado el tipo ${match.nombre || nameToSend}`
          );
        }
      }
    } catch (e: any) {
      console.error("Error creando tipo:", e);
      const msg = e?.toString?.() || "No se pudo crear el tipo de accesorio.";
      showError("Error", msg);
    } finally {
      setCreating(false);
    }
  }

  async function saveToOrden() {
    if (!ordenId) return;
    try {
      await invoke("update_accesorios_orden", {
        orden_id: ordenId,
        accesorios: selected,
      });
    } catch (e) {
      console.error("Error guardando accesorios en orden:", e);
    }
  }

  return (
    <div>
      <div className="mb-2">
        <div className="text-sm font-medium mb-1">Seleccionar accesorios</div>
        <div>
          <select
            value={""}
            onChange={(e) => {
              const val = parseInt(e.target.value);
              if (!isNaN(val)) addTipoToSelected(val);
            }}
            className="border rounded px-2 py-1 w-full"
          >
            <option value="">-- Seleccionar accesorio --</option>
            {tipos
              .filter(
                (t) => !selected.find((s) => s.tipo_accesorio_id === t.tipo_id)
              )
              .map((t) => (
                <option key={t.tipo_id} value={t.tipo_id}>
                  {t.nombre || `Tipo ${t.tipo_id}`}
                </option>
              ))}
          </select>
        </div>
      </div>

      <div className="mb-2">
        <label className="text-sm">Agregar nuevo tipo</label>
        <div className="flex gap-2 mt-1">
          <input
            value={newTipoName}
            onChange={(e) => setNewTipoName(e.target.value)}
            className="border px-2 py-1 rounded"
          />
          <Button
            type="button"
            onClick={handleCreateTipo}
            disabled={creating}
            className="cursor-pointer"
          >
            {creating ? "Creando..." : "Crear"}
          </Button>
        </div>
      </div>

      <div>
        <div className="text-sm font-medium mb-1">Accesorios seleccionados</div>
        <div className="flex gap-2 flex-wrap">
          {selected.map((s) => {
            const tipo = tipos.find((t) => t.tipo_id === s.tipo_accesorio_id);
            return (
              <div
                key={s.tipo_accesorio_id}
                className="px-2 py-1 bg-gray-100 rounded flex items-center gap-2"
              >
                <span>{tipo?.nombre || s.tipo_accesorio_id}</span>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => removeTipoFromSelected(s.tipo_accesorio_id)}
                  className="text-red-600"
                >
                  ✕
                </Button>
              </div>
            );
          })}
        </div>
      </div>

      {ordenId ? (
        <div className="mt-2">
          <Button
            type="button"
            onClick={saveToOrden}
            className="cursor-pointer"
          >
            Guardar accesorios en orden
          </Button>
        </div>
      ) : (
        <div className="mt-2 text-sm text-gray-600">
          Los accesorios se guardarán cuando la orden sea creada.
        </div>
      )}
    </div>
  );
}
