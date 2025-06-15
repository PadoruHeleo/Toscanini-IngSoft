import { Alert, AlertDescription } from "@/components/ui/alert";
import {
  IconAlertCircle,
  IconLock,
  IconWifi,
  IconUserX,
} from "@tabler/icons-react";

interface LoginErrorProps {
  error: string;
  className?: string;
}

export function LoginError({ error, className = "" }: LoginErrorProps) {
  const getErrorConfig = (errorCode: string) => {
    switch (errorCode) {
      case "USER_NOT_FOUND":
        return {
          icon: <IconLock className="h-4 w-4" />,
          title: "Credenciales incorrectas",
          message:
            "El email o la contraseña son incorrectos. Por favor, verifica tus datos e intenta nuevamente.",
          bgColor: "bg-red-50 border-red-200",
          textColor: "text-red-800",
          iconColor: "text-red-600",
        };
      case "INVALID_PASSWORD":
        return {
          icon: <IconLock className="h-4 w-4" />,
          title: "Credenciales incorrectas",
          message:
            "El email o la contraseña son incorrectos. Por favor, verifica tus datos e intenta nuevamente.",
          bgColor: "bg-red-50 border-red-200",
          textColor: "text-red-800",
          iconColor: "text-red-600",
        };
      case "USER_NO_PASSWORD":
        return {
          icon: <IconLock className="h-4 w-4" />,
          title: "Credenciales incorrectas",
          message:
            "El email o la contraseña son incorrectos. Por favor, verifica tus datos e intenta nuevamente.",
          bgColor: "bg-red-50 border-red-200",
          textColor: "text-red-800",
          iconColor: "text-red-600",
        };
      case "INVALID_CREDENTIALS":
        return {
          icon: <IconLock className="h-4 w-4" />,
          title: "Credenciales incorrectas",
          message:
            "El email o la contraseña son incorrectos. Por favor, verifica tus datos e intenta nuevamente.",
          bgColor: "bg-red-50 border-red-200",
          textColor: "text-red-800",
          iconColor: "text-red-600",
        };
      case "EMAIL_NOT_REGISTERED":
        return {
          icon: <IconUserX className="h-4 w-4" />,
          title: "Correo no registrado",
          message:
            "Este correo electrónico no está registrado en nuestro sistema. Verifica que hayas ingresado el correo correcto o contacta al administrador.",
          bgColor: "bg-yellow-50 border-yellow-200",
          textColor: "text-yellow-800",
          iconColor: "text-yellow-600",
        };
      case "EMAIL_SERVICE_ERROR":
        return {
          icon: <IconWifi className="h-4 w-4" />,
          title: "Error del servicio de correo",
          message:
            "No se pudo enviar el correo de recuperación. Por favor, intenta nuevamente en unos minutos.",
          bgColor: "bg-orange-50 border-orange-200",
          textColor: "text-orange-800",
          iconColor: "text-orange-600",
        };
      case "Database error":
      case "NETWORK_ERROR":
        return {
          icon: <IconWifi className="h-4 w-4" />,
          title: "Error de conexión",
          message:
            "No se pudo conectar con el servidor. Verifica tu conexión a internet e intenta nuevamente.",
          bgColor: "bg-orange-50 border-orange-200",
          textColor: "text-orange-800",
          iconColor: "text-orange-600",
        };
      default:
        return {
          icon: <IconAlertCircle className="h-4 w-4" />,
          title: "Error de inicio de sesión",
          message:
            "Ha ocurrido un error inesperado. Intenta nuevamente en unos momentos.",
          bgColor: "bg-red-50 border-red-200",
          textColor: "text-red-800",
          iconColor: "text-red-600",
        };
    }
  };

  const config = getErrorConfig(error);

  return (
    <Alert
      variant="destructive"
      className={`${config.bgColor} ${config.textColor} border-l-4 animate-in slide-in-from-top-2 duration-200 ${className}`}
    >
      <div className={`${config.iconColor} flex-shrink-0`}>{config.icon}</div>
      <AlertDescription className="ml-2">
        <div className="space-y-1">
          <div className="font-medium text-sm">{config.title}</div>
          <div className="text-xs opacity-90">{config.message}</div>
        </div>
      </AlertDescription>
    </Alert>
  );
}
