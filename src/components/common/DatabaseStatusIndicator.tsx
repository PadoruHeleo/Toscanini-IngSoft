import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  CheckCircle2,
  XCircle,
  Loader2,
  Cloud,
  AlertTriangle,
  Database,
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

export function DatabaseStatusIndicator() {
  const [status, setStatus] = useState<DatabaseStatus | null>(null);
  const [appState, setAppState] = useState<AppState | null>(null);
  const [isChecking, setIsChecking] = useState(false);

  const checkStatus = async () => {
    try {
      setIsChecking(true);

      // Fetch both statuses
      const [dbResult, stateResult] = await Promise.all([
        invoke<DatabaseStatus>("get_database_status"),
        invoke<AppState>("get_app_state"),
      ]);

      setStatus(dbResult);
      setAppState(stateResult);
    } catch (error) {
      console.error("Error checking status:", error);
      // Mantener el estado anterior si falla una comprobación puntual para evitar parpadeos
    } finally {
      setIsChecking(false);
    }
  };

  useEffect(() => {
    checkStatus();
    const interval = setInterval(checkStatus, 30000);
    return () => clearInterval(interval);
  }, []);

  if (!status || !appState) {
    return (
      <div className="fixed bottom-4 right-4 z-50 flex items-center gap-2 px-2 py-1.5 bg-gray-100 border border-gray-300 rounded-md shadow-sm opacity-50">
        <Loader2 className="h-3 w-3 animate-spin text-gray-500" />
      </div>
    );
  }

  // Determine display properties based on state
  let config = {
    bg: "bg-gray-50",
    border: "border-gray-200",
    text: "text-gray-700",
    icon: <Loader2 className="h-3 w-3 animate-spin" />,
    label: "Cargando...",
    tooltip: "Verificando sistema...",
  };

  if (appState.use_api) {
    if (appState.is_fallback_mode) {
      config = {
        bg: "bg-amber-50",
        border: "border-amber-200 hover:bg-amber-100",
        text: "text-amber-700",
        icon: <AlertTriangle className="h-3 w-3 text-amber-600" />,
        label: "Modo Recuperación",
        tooltip:
          "Base de datos local falló. Usando API remota (Modo Fallback).",
      };
    } else {
      config = {
        bg: "bg-blue-50",
        border: "border-blue-200 hover:bg-blue-100",
        text: "text-blue-700",
        icon: <Cloud className="h-3 w-3 text-blue-600" />,
        label: "Modo API",
        tooltip: "Sistema operando en modo API (Nube)",
      };
    }
  } else {
    // Database Mode
    if (status.is_connected) {
      config = {
        bg: "bg-green-50",
        border: "border-green-200 hover:bg-green-100",
        text: "text-green-700",
        icon: <CheckCircle2 className="h-3 w-3 text-green-600" />,
        label: "DB Local",
        tooltip: "Base de datos local conectada",
      };
    } else {
      config = {
        bg: "bg-red-50",
        border: "border-red-200 hover:bg-red-100",
        text: "text-red-700",
        icon: <XCircle className="h-3 w-3 text-red-600" />,
        label: "DB Error",
        tooltip: status.error_message || "Error de conexión a BD Local",
      };
    }
  }

  return (
    <div
      className={`fixed bottom-10 right-14 z-50 flex items-center gap-2 px-2 py-1.5 rounded-md shadow-sm border transition-all cursor-help ${config.bg} ${config.border}`}
      title={config.tooltip}
    >
      {isChecking ? (
        <Loader2 className="h-3 w-3 animate-spin text-gray-500" />
      ) : (
        config.icon
      )}
      <span className={`text-xs font-medium ${config.text}`}>
        {config.label}
      </span>
      {/* Pulse effect for error states */}
      {(!status.is_connected && !appState.use_api) ||
      appState.is_fallback_mode ? (
        <div
          className={`h-1.5 w-1.5 rounded-full animate-pulse ${
            appState.is_fallback_mode ? "bg-amber-500" : "bg-red-500"
          }`}
        />
      ) : null}
    </div>
  );
}
