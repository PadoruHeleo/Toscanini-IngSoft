import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

interface Equipo {
  equipo_id: number;
  numero_serie?: string;
}

interface OrdenTrabajo {
  id?: number;
  orden_id?: number;
  created_at: string;
  finished_at: string | null;
  estado: string;
  informe_id?: number;
}

interface Informe {
  id: number;
  informe_codigo: string;
  diagnostico: string[];
  created_at: string;
  solucion_aplicada: string;
  tecnico_responsable: string;
}

function formatChileanDate(dateString: string) {
  if (!dateString) return "";
  const date = new Date(dateString);
  return date.toLocaleString("es-CL", { timeZone: "America/Santiago" });
}

function capitalize(str: string) {
  if (!str) return "";
  return str.charAt(0).toUpperCase() + str.slice(1);
}

export function EquipoHistorialDialog({
  open,
  onOpenChange,
  equipo,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  equipo: Equipo | null;
}) {
  const [ordenes, setOrdenes] = useState<OrdenTrabajo[]>([]);
  const [loading, setLoading] = useState(false);
  const [informe, setInforme] = useState<Informe | null>(null);

  // Estado para el segundo dialog
  const [openSubDialog, setOpenSubDialog] = useState(false);

  useEffect(() => {
    if (open && equipo) {
      setLoading(true);
      invoke<OrdenTrabajo[]>("get_ordenes_trabajo_by_equipo", {
        equipoId: equipo.equipo_id,
      })
        .then((data) => {
          console.log("Ordenes recibidas:", data);
          setOrdenes(data);
        })
        .catch(() => {
          setOrdenes([]);
        })
        .finally(() => setLoading(false));
    } else {
      setOrdenes([]);
    }
  }, [open, equipo]);

  useEffect(() => {
    if (ordenes.length > 0 && ordenes[0].informe_id) {
      invoke<any>("get_informe_by_id", {
        informeId: ordenes[0].informe_id,
      })
        .then((data) => {
          console.log("Informe recibido:", data);
          setInforme(data);
        })
        .catch(() => setInforme(null));
    } else {
      setInforme(null);
      console.log("No hay informe asociado o no se pudo obtener.");
    }
  }, [ordenes]);

  if (!equipo) return null;

  return (
    <>
      {/* Dialog principal */}
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent style={{ minWidth: "800px" }}>
          <DialogHeader>
            <DialogTitle>Historial de Equipo</DialogTitle>
            <DialogDescription>
              Permite revisar el historial del equipo seleccionado.
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4">
            <div>
              <strong>Número de Serie:</strong>{" "}
              {equipo.numero_serie || "N/A"}
              <br />
              <strong>ID del equipo:</strong> {equipo.equipo_id || "N/A"}
            </div>
            <Table>
              <TableCaption>Órdenes de trabajo asociadas</TableCaption>
              <TableHeader>
                <TableRow>
                  <TableHead>ID Orden</TableHead>
                  <TableHead>Fecha de Ingreso</TableHead>
                  <TableHead>Estado</TableHead>
                  <TableHead>Fecha de Finalizacion</TableHead>
                  <TableHead>Ver informe</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {loading ? (
                  <TableRow>
                    <TableCell colSpan={9}>Cargando...</TableCell>
                  </TableRow>
                ) : ordenes.length === 0 ? (
                  <TableRow>
                    <TableCell colSpan={9}>
                      No hay órdenes de trabajo.
                    </TableCell>
                  </TableRow>
                ) : (
                  ordenes.map((orden) => (
                    <TableRow key={orden.id}>
                      <TableCell>{orden.id ?? orden.orden_id}</TableCell>
                      <TableCell>
                        {formatChileanDate(orden.created_at)}
                      </TableCell>
                      <TableCell>
                        {capitalize(orden.estado ?? "")}
                      </TableCell>
                      <TableCell>
                        {formatChileanDate(orden.created_at)}
                      </TableCell>
                      <TableCell>
                        <Button
                          variant="outline"
                          onClick={() => setOpenSubDialog(true)}
                          disabled={!orden.informe_id}
                        >
                          Ver informe
                        </Button>
                      </TableCell>
                    </TableRow>
                  ))
                )}
              </TableBody>
            </Table>
            <DialogFooter>
              <Button onClick={() => onOpenChange(false)} variant="outline">
                Cerrar
              </Button>
            </DialogFooter>
          </div>
        </DialogContent>
      </Dialog>

      <Dialog open={openSubDialog} onOpenChange={setOpenSubDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Informe de Orden de Trabajo</DialogTitle>
          </DialogHeader>
          <div>
            {informe ? (
              <div>
                <p><strong>Código:</strong> {informe.informe_codigo}</p>
                <p><strong>Diagnóstico:</strong> {informe.diagnostico}</p>
                <p><strong>Técnico Responsable:</strong> {informe.tecnico_responsable}</p>
                <p><strong>Solución Aplicada:</strong> {informe.solucion_aplicada}</p>
                <p><strong>Fecha de Creación:</strong> {formatChileanDate(informe.created_at)}</p>
              </div>
            ) : (
              <p>No hay informe disponible.</p>
            )}
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}
