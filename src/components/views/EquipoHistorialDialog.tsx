import { Button } from "@/components/ui/button";
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

const historialEjemplo = [
  {
    fecha: "2024-01-01",
    descripcion: "Ingreso a taller",
    estado: "Recibido",
  },
  {
    fecha: "2024-01-05",
    descripcion: "Reparación iniciada",
    estado: "En proceso",
  },
  {
    fecha: "2024-01-10",
    descripcion: "Reparación finalizada",
    estado: "Listo",
  },
];

export function EquipoHistorialDialog({
  open,
  onOpenChange,
  equipo,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  equipo: Equipo | null;
}) {
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
          </div>
          <Table>
            <TableCaption>Historial de movimientos</TableCaption>
            <TableHeader>
              <TableRow>
                <TableHead>Fecha</TableHead>
                <TableHead>Descripción</TableHead>
                <TableHead>Estado</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {historialEjemplo.map((item, idx) => (
                <TableRow key={idx}>
                  <TableCell>{item.fecha}</TableCell>
                  <TableCell>{item.descripcion}</TableCell>
                  <TableCell>{item.estado}</TableCell>
                </TableRow>
              ))}
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