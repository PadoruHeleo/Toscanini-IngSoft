import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { CheckCircle, XCircle, RefreshCw } from "lucide-react";

interface EquipoEstadoSistemaProps {
  equipoId: number;
  showRefresh?: boolean;
  size?: "sm" | "md" | "lg";
}

interface EstadoSistema {
  enSistema: boolean;
  mensaje: string;
}

export function EquipoEstadoSistema({
  equipoId,
  showRefresh = false,
}: EquipoEstadoSistemaProps) {
  const [estado, setEstado] = useState<EstadoSistema | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const checkEstado = async () => {
    if (!equipoId) return;

    setLoading(true);
    setError(null);

    try {
      const [enSistema, mensaje] = await invoke<[boolean, string]>(
        "equipo_esta_en_sistema",
        { equipoId }
      );

      setEstado({
        enSistema,
        mensaje,
      });
    } catch (err) {
      console.error("Error verificando estado del equipo:", err);
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    checkEstado();
  }, [equipoId]);

  if (loading) {
    return (
      <Badge variant="secondary" className="flex items-center gap-1">
        <RefreshCw className="h-3 w-3 animate-spin" />
        Verificando...
      </Badge>
    );
  }

  if (error) {
    return (
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <Badge variant="destructive" className="flex items-center gap-1">
              <XCircle className="h-3 w-3" />
              Error
            </Badge>
          </TooltipTrigger>
          <TooltipContent>
            <p>{error}</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    );
  }

  if (!estado) {
    return <Badge variant="secondary">Sin datos</Badge>;
  }

  const badgeVariant = estado.enSistema ? "default" : "secondary";
  const icon = estado.enSistema ? (
    <CheckCircle className="h-3 w-3 text-green-600" />
  ) : (
    <XCircle className="h-3 w-3 text-red-600" />
  );

  const texto = estado.enSistema ? "En Sistema" : "Fuera Sistema";

  const badgeClassName = estado.enSistema
    ? "bg-green-100 text-green-800 border-green-200 hover:bg-green-200"
    : "bg-red-100 text-red-800 border-red-200 hover:bg-red-200";

  return (
    <div className="flex items-center gap-2">
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <Badge
              variant={badgeVariant}
              className={`flex items-center gap-1 ${badgeClassName}`}
            >
              {icon}
              {texto}
            </Badge>
          </TooltipTrigger>
          <TooltipContent>
            <p>{estado.mensaje}</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>

      {showRefresh && (
        <Button
          variant="ghost"
          size="sm"
          onClick={checkEstado}
          disabled={loading}
          className="h-6 w-6 p-0"
        >
          <RefreshCw className={`h-3 w-3 ${loading ? "animate-spin" : ""}`} />
        </Button>
      )}
    </div>
  );
}

// Hook para usar el estado del sistema en otros componentes
export function useEquipoEstadoSistema(equipoId: number) {
  const [estado, setEstado] = useState<EstadoSistema | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const checkEstado = async () => {
    if (!equipoId) return;

    setLoading(true);
    setError(null);

    try {
      const [enSistema, mensaje] = await invoke<[boolean, string]>(
        "equipo_esta_en_sistema",
        { equipoId }
      );

      setEstado({
        enSistema,
        mensaje,
      });
    } catch (err) {
      console.error("Error verificando estado del equipo:", err);
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    checkEstado();
  }, [equipoId]);

  return {
    estado,
    loading,
    error,
    refetch: checkEstado,
  };
}
