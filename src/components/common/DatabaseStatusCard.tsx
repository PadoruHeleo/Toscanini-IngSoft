import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  CheckCircle2,
  XCircle,
  Loader2,
  Cloud,
  AlertTriangle,
} from "lucide-react";

interface DatabaseStatus {
  is_connected: boolean;
  error_message?: string;
  last_check?: string;
}

interface AppState {
  use_api: boolean;
  is_fallback_mode: boolean;
}

export function DatabaseStatusCard() {
  const [status, setStatus] = useState<DatabaseStatus | null>(null);
  const [appState, setAppState] = useState<AppState | null>(null);
  const [isChecking, setIsChecking] = useState(false);

  const checkDatabaseStatus = async () => {
    try {
      setIsChecking(true);
      const [dbResult, stateResult] = await Promise.all([
        invoke<DatabaseStatus>("get_database_status"),
        invoke<AppState>("get_app_state"),
      ]);
      setStatus(dbResult);
      setAppState(stateResult);
    } catch (error) {
      console.error("Error checking database status:", error);
      setStatus({
        is_connected: false,
        error_message: "Error al verificar el estado de la base de datos",
      });
    } finally {
      setIsChecking(false);
    }
  };

  useEffect(() => {
    checkDatabaseStatus();
    // Verificar el estado cada 30 segundos
    const interval = setInterval(checkDatabaseStatus, 30000);
    return () => clearInterval(interval);
  }, []);

  if (!status || !appState) {
    return (
      <div className="p-3 bg-gray-50 border border-gray-200 rounded-lg mb-4 flex items-center gap-2">
        <Loader2 className="h-4 w-4 animate-spin text-gray-500" />
        <span className="text-sm text-gray-600">Verificando conexión...</span>
      </div>
    );
  }

  // Determine styles and content based on state
  let config = {
    bg: "bg-gray-50 border-gray-200",
    icon: <Loader2 className="h-4 w-4 animate-spin text-gray-500" />,
    textClass: "text-gray-800",
    statusText: "Verificando...",
    subText: null as React.ReactNode,
  };

  if (appState.use_api) {
    if (appState.is_fallback_mode) {
      config = {
        bg: "bg-amber-50 border-amber-200",
        icon: <AlertTriangle className="h-4 w-4 text-amber-600" />,
        textClass: "text-amber-800",
        statusText: "Modo Recuperación",
        subText: (
          <span className="text-xs text-amber-600">
            Usando API (DB Local falló)
          </span>
        ),
      };
    } else {
      config = {
        bg: "bg-blue-50 border-blue-200",
        icon: <Cloud className="h-4 w-4 text-blue-600" />,
        textClass: "text-blue-800",
        statusText: "Modo API",
        subText: (
          <span className="text-xs text-blue-600">Conectado a la Nube</span>
        ),
      };
    }
  } else {
    // DB Mode
    if (status.is_connected) {
      config = {
        bg: "bg-green-50 border-green-200",
        icon: <CheckCircle2 className="h-4 w-4 text-green-600" />,
        textClass: "text-green-800",
        statusText: "Base de Datos: Conectada",
        subText: null,
      };
    } else {
      config = {
        bg: "bg-red-50 border-red-200",
        icon: <XCircle className="h-4 w-4 text-red-600" />,
        textClass: "text-red-800",
        statusText: "Base de Datos: Desconectada",
        subText: status.error_message && (
          <p className="text-xs text-red-600 mt-1">
            {status.error_message.length > 50
              ? `${status.error_message.substring(0, 50)}...`
              : status.error_message}
          </p>
        ),
      };
    }
  }

  return (
    <div
      className={`p-3 border rounded-lg mb-4 flex items-center justify-between ${config.bg}`}
    >
      <div className="flex items-center gap-2">
        {config.icon}
        <div>
          <p className={`text-sm font-medium ${config.textClass}`}>
            {config.statusText}
          </p>
          {config.subText}
        </div>
      </div>
      {isChecking && <Loader2 className="h-4 w-4 animate-spin text-gray-500" />}
    </div>
  );
}
