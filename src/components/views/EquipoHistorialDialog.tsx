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
  informe_id?: number
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
  const [informe, setInforme] = useState<any>(null);

  useEffect(() => {
    if (open && equipo) {
      setLoading(true);
      invoke<OrdenTrabajo[]>("get_ordenes_trabajo_by_equipo", { equipoId: equipo.equipo_id })
        .then((data) => {
          console.log("Ordenes recibidas:", data);
          setOrdenes(data);
        })
        .catch(() => { setOrdenes([]) })
        .finally(() => setLoading(false));
    } else {
      setOrdenes([]);
    }
  }, [open, equipo]);


  useEffect(() => {
    if (ordenes.length > 0 && ordenes[0].informe_id) {
      invoke<any>("get_informe_by_id", { informe_id: ordenes[0].informe_id })
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
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Historial de Equipo</DialogTitle>
          <DialogDescription>
            Permite revisar el historial del equipo seleccionado.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4">
          <div>
            <strong>Número de Serie:</strong> {equipo.numero_serie || "N/A"}
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
                <TableHead>ID Informe</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {loading ? (
                <TableRow>
                  <TableCell colSpan={3}>Cargando...</TableCell>
                </TableRow>
              ) : ordenes.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={3}>No hay órdenes de trabajo.</TableCell>
                </TableRow>
              ) : (
                ordenes.map((orden) => (
                  <TableRow key={orden.id}>
                    <TableCell>{orden.id ?? orden.orden_id}</TableCell>
                    <TableCell>{orden.created_at}</TableCell>
                    <TableCell>{capitalize(orden.estado ?? "")}</TableCell>
                    <TableCell>{orden.finished_at || "En progreso"}</TableCell>
                    <TableCell>{orden.informe_id ?? "No Aplica"}</TableCell>
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
  );
}