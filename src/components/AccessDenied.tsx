import { AlertCircle, Lock, ArrowLeft } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useView } from "@/contexts/ViewContext";
import { useAuth } from "@/contexts/AuthContext";

interface AccessDeniedProps {
  requiredRole?: string;
  featureName?: string;
  showBackButton?: boolean;
}

export function AccessDenied({
  requiredRole = "administrador",
  featureName = "esta funcionalidad",
  showBackButton = true,
}: AccessDeniedProps) {
  const { setCurrentView } = useView();
  const { user } = useAuth();

  const handleGoBack = () => {
    setCurrentView("inicio");
  };

  const getRoleLabel = (role?: string) => {
    switch (role) {
      case "admin":
        return "Administrador";
      case "tecnico":
        return "Técnico";
      case "recepcion":
        return "Recepcionista";
      default:
        return "Usuario";
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center p-4">
      <Card className="w-full max-w-md">
        <CardHeader className="text-center">
          <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-red-100">
            <Lock className="h-6 w-6 text-red-600" />
          </div>
          <CardTitle className="flex items-center justify-center gap-2 text-xl">
            <AlertCircle className="h-5 w-5 text-red-600" />
            Acceso Denegado
          </CardTitle>
          <CardDescription className="text-center">
            No tienes permisos suficientes para acceder a {featureName}
          </CardDescription>
        </CardHeader>

        <CardContent className="space-y-4">
          <div className="rounded-lg bg-yellow-50 border border-yellow-200 p-4">
            <div className="text-sm text-yellow-800">
              <p className="font-medium mb-2">Información de acceso:</p>
              <ul className="space-y-1 text-xs">
                <li>
                  • <strong>Tu rol actual:</strong>{" "}
                  {getRoleLabel(user?.usuario_rol || undefined)}
                </li>
                <li>
                  • <strong>Rol requerido:</strong> {requiredRole}
                </li>
                <li>
                  • <strong>Usuario:</strong>{" "}
                  {user?.usuario_nombre || "No disponible"}
                </li>
              </ul>
            </div>
          </div>

          <div className="text-center text-sm text-gray-600">
            <p>
              Si crees que deberías tener acceso a esta funcionalidad, contacta
              a tu administrador del sistema.
            </p>
          </div>

          {showBackButton && (
            <div className="flex justify-center">
              <Button
                onClick={handleGoBack}
                variant="outline"
                className="flex items-center gap-2"
              >
                <ArrowLeft className="h-4 w-4" />
                Volver al Inicio
              </Button>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
