import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type TipoAccesorio = {
  tipo_id: number;
  nombre: string;
  created_at: string;
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

  useEffect(() => {
    loadTipos();
  }, []);

  useEffect(() => {
    setSelected(value || []);
  }, [value]);

  async function loadTipos() {
    try {
      const res = (await invoke("get_tipos_accesorios")) as TipoAccesorio[];
      setTipos(res || []);
    } catch (e) {
      console.error("Error cargando tipos de accesorios", e);
      setTipos([]);
    }
  }

  async function handleAddTipo(tipoId: number) {
    const exists = selected.find((s) => s.tipo_accesorio_id === tipoId);
    if (exists) return;
    const newAcc: OrdenAccesorio = {
      tipo_accesorio_id: tipoId,
      estado: "presente",
      observaciones: null,
    };
    const next = [...selected, newAcc];
    setSelected(next);
    onChange?.(next);
  }

  async function handleRemoveTipo(tipoId: number) {
    const next = selected.filter((s) => s.tipo_accesorio_id !== tipoId);
    setSelected(next);
    onChange?.(next);
  }

  async function handleCreateTipo() {
    if (!newTipoName.trim()) return;
    try {
      const id = (await invoke("create_tipo_accesorio", newTipoName)) as number;
      await loadTipos();
      setNewTipoName("");
      // auto add
      handleAddTipo(id);
    } catch (e) {
      console.error("Error creando tipo", e);
    }
  }

  async function handleSaveToOrden() {
    if (!ordenId) return;
    try {
      // Send accesorio list to backend to save
      await invoke("update_accesorios_orden", {
        orden_id: ordenId,
        accesorios: selected,
      });
    } catch (e) {
      console.error("Error guardando accesorios", e);
    }
  }

  return (
    <div className="accesorios-form">
      <div style={{ marginBottom: 8 }}>
        <label>Tipos disponibles</label>
        <div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
          {tipos.map((t) => (
            <button
              key={t.tipo_id}
              onClick={() => handleAddTipo(t.tipo_id)}
              className="btn btn-outline"
            >
              {t.nombre}
            </button>
          ))}
        </div>
      </div>

      <div style={{ marginTop: 8 }}>
        <label>Agregar nuevo tipo</label>
        <div style={{ display: "flex", gap: 8 }}>
          <input
            value={newTipoName}
            onChange={(e) => setNewTipoName(e.target.value)}
          />
          <button onClick={handleCreateTipo} className="btn btn-primary">
            Crear
          </button>
        </div>
      </div>

      <div style={{ marginTop: 12 }}>
        <label>Accesorios seleccionados</label>
        <ul>
          {selected.map((s) => {
            const tipo = tipos.find((t) => t.tipo_id === s.tipo_accesorio_id);
            return (
              <li key={s.tipo_accesorio_id}>
                <strong>{tipo?.nombre || s.tipo_accesorio_id}</strong>
                <button
                  onClick={() => handleRemoveTipo(s.tipo_accesorio_id)}
                  style={{ marginLeft: 8 }}
                >
                  Quitar
                </button>
                <div>
                  <input
                    placeholder="Observaciones"
                    value={s.observaciones || ""}
                    onChange={(e) => {
                      const next = selected.map((it) =>
                        it.tipo_accesorio_id === s.tipo_accesorio_id
                          ? { ...it, observaciones: e.target.value }
                          : it
                      );
                      setSelected(next);
                      onChange?.(next);
                    }}
                  />
                </div>
              </li>
            );
          })}
        </ul>
      </div>

      {ordenId ? (
        <div style={{ marginTop: 8 }}>
          <button onClick={handleSaveToOrden} className="btn btn-primary">
            Guardar accesorios en orden
          </button>
        </div>
      ) : (
        <div style={{ marginTop: 8, color: "#666" }}>
          Los accesorios se guardarán cuando la orden sea creada.
        </div>
      )}
    </div>
  );
}
