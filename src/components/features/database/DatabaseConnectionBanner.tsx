import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import {
  Database,
  AlertTriangle,
  RefreshCw,
  X,
  ChevronDown,
  Clock,
  Wifi,
  WifiOff,
  Server,
  AlertCircle,
  Cloud,
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

export function DatabaseConnectionBanner() {
  const [status, setStatus] = useState<DatabaseStatus | null>(null);
  const [appState, setAppState] = useState<AppState | null>(null);
  const [isRetrying, setIsRetrying] = useState(false);
  const [isChecking, setIsChecking] = useState(false);
  const [isDismissed, setIsDismissed] = useState(false);
  const [showDetails, setShowDetails] = useState(false);
  const [connectionAttempts, setConnectionAttempts] = useState(0);

  const checkDatabaseStatus = async (
    showLoading = false
  ): Promise<DatabaseStatus | null> => {
    try {
      if (showLoading) setIsChecking(true);

      const [dbResult, stateResult] = await Promise.all([
        invoke<DatabaseStatus>("get_database_status"),
        invoke<AppState>("get_app_state"),
      ]);

      setStatus(dbResult);
      setAppState(stateResult);

      // Si la conexión se restaura (o estamos en modo API sin fallback), resetear
      if (dbResult.is_connected && isDismissed) {
        setIsDismissed(false);
        setConnectionAttempts(0);
      }

      return dbResult;
    } catch (error) {
      console.error("Error checking database status:", error);
      const errorStatus: DatabaseStatus = {
        is_connected: false,
        error_message: "Error al verificar el estado de la base de datos",
      };
      setStatus(errorStatus);
      return errorStatus;
    } finally {
      if (showLoading) setIsChecking(false);
    }
  };

  const retryConnection = async () => {
    try {
      setIsRetrying(true);
      setConnectionAttempts((prev) => prev + 1);

      // Si estamos en fallback, intentar reconectar a DB implica retry normal
      const result = await invoke<DatabaseStatus>("retry_database_connection");
      setStatus(result);

      if (result.is_connected) {
        setIsDismissed(false);
        setConnectionAttempts(0);
        setShowDetails(false);
        // Tambien actualizar appState por si cambia de fallback a normal
        const stateResult = await invoke<AppState>("get_app_state");
        setAppState(stateResult);
      }
    } catch (error) {
      console.error("Error retrying database connection:", error);
      setStatus((prev) => ({
        is_connected: false,
        error_message:
          "Error al intentar reconectar. Verifica la configuración de la base de datos.",
        last_check: prev?.last_check,
      }));
    } finally {
      setIsRetrying(false);
    }
  };

  const getErrorType = (errorMessage?: string) => {
    if (!errorMessage) return "unknown";

    const message = errorMessage.toLowerCase();
    if (
      message.includes("connection refused") ||
      message.includes("denegó expresamente")
    ) {
      return "connection_refused";
    }
    if (message.includes("timeout") || message.includes("time out")) {
      return "timeout";
    }
    if (
      message.includes("authentication") ||
      message.includes("access denied")
    ) {
      return "auth";
    }
    if (
      message.includes("database") &&
      message.includes("not") &&
      message.includes("exist")
    ) {
      return "database_not_exist";
    }
    if (message.includes("api")) {
      return "api_mode";
    }
    return "unknown";
  };

  const getErrorSolution = (errorType: string) => {
    switch (errorType) {
      case "connection_refused":
        return "Verifica que el servidor MySQL esté ejecutándose y accesible en el puerto configurado.";
      case "timeout":
        return "El servidor responde lentamente. Verifica la conexión de red y la configuración del servidor.";
      case "auth":
        return "Credenciales incorrectas. Verifica el usuario y contraseña en la configuración.";
      case "database_not_exist":
        return "La base de datos especificada no existe. Créala o verifica el nombre en la configuración.";
      case "api_mode":
        return "El sistema está configurado para usar la API remota. Esto es normal si no se requiere base de datos local.";
      default:
        return "Verifica la configuración de conexión en el archivo .env y que el servidor MySQL esté disponible.";
    }
  };
  useEffect(() => {
    checkDatabaseStatus();

    // Verificar el estado cada 30 segundos
    const interval = setInterval(async () => {
      await checkDatabaseStatus(false);
      // Auto-reconnect logic...
    }, 30000);

    return () => clearInterval(interval);
  }, [isRetrying]);

  // Si no hay status o appState, no mostramos nada aun
  if (!status || !appState) {
    return null;
  }

  // Si estamos conectados, no mostrar
  if (status.is_connected) {
    return null;
  }

  // Si estamos descartados
  if (isDismissed) {
    return null;
  }

  // LOGICA PRINCIPAL DE VISUALIZACION

  // Caso 1: Modo API Puro (Configurado para usar API, NO es fallback)
  // En este caso, no mostramos error de base de datos porque es el comportamiento esperado.
  if (appState.use_api && !appState.is_fallback_mode) {
    return null;
  }

  // Caso 2: Modo Fallback (Configurado DB pero falló -> Usando API)
  // Mostramos Warning en lugar de Error
  if (appState.is_fallback_mode) {
    return (
      <div className="w-full p-4 border-b bg-amber-50 dark:bg-amber-950/20">
        <Card className="border-amber-200 dark:border-amber-800">
          <CardContent className="p-4">
            <div className="flex items-start gap-4">
              {/* Icono de estado */}
              <div className="flex-shrink-0 mt-1">
                <div className="relative">
                  <Cloud className="h-6 w-6 text-amber-600 dark:text-amber-400" />
                  <div className="absolute -bottom-1 -right-1">
                    <AlertTriangle className="h-3 w-3 text-amber-500 bg-white dark:bg-gray-900 rounded-full" />
                  </div>
                </div>
              </div>

              {/* Contenido principal */}
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2 mb-2">
                  <AlertTriangle className="h-4 w-4 text-amber-600 dark:text-amber-400" />
                  <h3 className="font-semibold text-amber-900 dark:text-amber-100">
                    Modo Recuperación Activo
                  </h3>
                  <Badge
                    variant="outline"
                    className="text-xs border-amber-500 text-amber-700"
                  >
                    Usando API
                  </Badge>
                </div>
                <p className="text-sm text-amber-800 dark:text-amber-200 mb-3">
                  La base de datos local no está disponible. El sistema está
                  operando en <strong>Modo Recuperación</strong> utilizando la
                  API remota.
                </p>

                <div className="space-y-2">
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => setShowDetails(!showDetails)}
                    className="p-0 h-auto text-amber-700 dark:text-amber-300 hover:text-amber-900 dark:hover:text-amber-100"
                  >
                    <ChevronDown
                      className={`h-4 w-4 mr-1 transition-transform ${
                        showDetails ? "rotate-180" : ""
                      }`}
                    />
                    Ver detalles de conexión
                  </Button>
                  {showDetails && (
                    <div className="mt-3 bg-amber-100 dark:bg-amber-900/30 rounded-lg p-3 space-y-2">
                      <div className="flex items-center gap-2 text-xs text-amber-700 dark:text-amber-300">
                        <AlertCircle className="h-3 w-3" />
                        <span className="font-medium">Error Original:</span>
                        <code className="bg-amber-200 dark:bg-amber-800 px-1 rounded text-xs">
                          {status.error_message || "Conexión rechazada"}
                        </code>
                      </div>
                      <div className="mt-2 text-xs text-amber-800">
                        El sistema intentará reconectar a la base de datos local
                        periódicamente o puede intentar manualmente.
                      </div>
                    </div>
                  )}
                </div>
              </div>

              {/* Acciones */}
              <div className="flex-shrink-0 flex items-center gap-2">
                <Button
                  onClick={retryConnection}
                  disabled={isRetrying}
                  variant="default"
                  size="sm"
                  className="bg-amber-600 hover:bg-amber-700 text-white border-none"
                >
                  {isRetrying ? (
                    <RefreshCw className="h-3 w-3 mr-1 animate-spin" />
                  ) : (
                    <RefreshCw className="h-3 w-3 mr-1" />
                  )}
                  Reconectar DB
                </Button>

                <Button
                  onClick={() => setIsDismissed(true)}
                  variant="ghost"
                  size="sm"
                  className="text-amber-600 hover:text-amber-800 hover:bg-amber-100 dark:text-amber-400 dark:hover:text-amber-200 dark:hover:bg-amber-900/30 p-2"
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    );
  }

  // Caso 3: Error de Base de Datos (Modo DB, pero desconectado)
  const errorType = getErrorType(status.error_message);
  const solution = getErrorSolution(errorType);
  const lastCheck = status.last_check ? new Date(status.last_check) : null;

  return (
    <div className="w-full p-4 border-b bg-red-50 dark:bg-red-950/20">
      <Card className="border-red-200 dark:border-red-800">
        <CardContent className="p-4">
          <div className="flex items-start gap-4">
            {/* Icono de estado */}
            <div className="flex-shrink-0 mt-1">
              <div className="relative">
                <Server className="h-6 w-6 text-red-600 dark:text-red-400" />
                <div className="absolute -bottom-1 -right-1">
                  <WifiOff className="h-3 w-3 text-red-500 bg-white dark:bg-gray-900 rounded-full" />
                </div>
              </div>
            </div>

            {/* Contenido principal */}
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2 mb-2">
                <AlertTriangle className="h-4 w-4 text-red-600 dark:text-red-400" />
                <h3 className="font-semibold text-red-900 dark:text-red-100">
                  Base de Datos Desconectada
                </h3>
                <Badge variant="destructive" className="text-xs">
                  {connectionAttempts > 0
                    ? `${connectionAttempts} intentos`
                    : "Sin conexión"}
                </Badge>
              </div>
              <p className="text-sm text-red-800 dark:text-red-200 mb-3">
                No se puede establecer conexión con la base de datos. Algunas
                funcionalidades pueden no estar disponibles.
              </p>{" "}
              {/* Detalles expandibles */}
              <div className="space-y-2">
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => setShowDetails(!showDetails)}
                  className="p-0 h-auto text-red-700 dark:text-red-300 hover:text-red-900 dark:hover:text-red-100"
                >
                  <ChevronDown
                    className={`h-4 w-4 mr-1 transition-transform ${
                      showDetails ? "rotate-180" : ""
                    }`}
                  />
                  Ver detalles técnicos
                </Button>

                {showDetails && (
                  <div className="mt-3 bg-red-100 dark:bg-red-900/30 rounded-lg p-3 space-y-2">
                    <div className="flex items-center gap-2 text-xs text-red-700 dark:text-red-300">
                      <AlertCircle className="h-3 w-3" />
                      <span className="font-medium">Error:</span>
                      <code className="bg-red-200 dark:bg-red-800 px-1 rounded text-xs">
                        {status.error_message || "Error desconocido"}
                      </code>
                    </div>

                    {lastCheck && (
                      <div className="flex items-center gap-2 text-xs text-red-600 dark:text-red-400">
                        <Clock className="h-3 w-3" />
                        <span>
                          Última verificación: {lastCheck.toLocaleString()}
                        </span>
                      </div>
                    )}

                    <div className="mt-2 p-2 bg-red-50 dark:bg-red-900/50 rounded border border-red-200 dark:border-red-800">
                      <p className="text-xs text-red-800 dark:text-red-200">
                        <strong>Solución sugerida:</strong> {solution}
                      </p>
                    </div>
                  </div>
                )}
              </div>
            </div>

            {/* Acciones */}
            <div className="flex-shrink-0 flex items-center gap-2">
              <Button
                onClick={() => checkDatabaseStatus(true)}
                disabled={isChecking}
                variant="outline"
                size="sm"
                className="border-red-300 text-red-700 hover:bg-red-100 dark:border-red-700 dark:text-red-300 dark:hover:bg-red-900/30"
              >
                {isChecking ? (
                  <RefreshCw className="h-3 w-3 mr-1 animate-spin" />
                ) : (
                  <Wifi className="h-3 w-3 mr-1" />
                )}
                Verificar
              </Button>

              <Button
                onClick={retryConnection}
                disabled={isRetrying}
                variant="default"
                size="sm"
                className="bg-red-600 hover:bg-red-700 text-white"
              >
                {isRetrying ? (
                  <RefreshCw className="h-3 w-3 mr-1 animate-spin" />
                ) : (
                  <Database className="h-3 w-3 mr-1" />
                )}
                Reconectar
              </Button>

              <Button
                onClick={() => setIsDismissed(true)}
                variant="ghost"
                size="sm"
                className="text-red-600 hover:text-red-800 hover:bg-red-100 dark:text-red-400 dark:hover:text-red-200 dark:hover:bg-red-900/30 p-2"
              >
                <X className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
