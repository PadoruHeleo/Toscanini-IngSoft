import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useView } from "@/contexts/ViewContext";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Alert, AlertDescription } from "../ui/alert";
import { Label } from "@/components/ui/label";

interface RequestPasswordResetRequest {
  usuario_correo: string;
}

interface ResetPasswordRequest {
  reset_code: string;
  nueva_contrasena: string;
}

export function PasswordResetView() {
  const [step, setStep] = useState<"request" | "verify" | "reset" | "success">(
    "request"
  );
  const [email, setEmail] = useState("");
  const [resetCode, setResetCode] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState("");
  const [error, setError] = useState("");
  const { setCurrentView } = useView();

  const handleRequestReset = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError("");
    setMessage("");

    try {
      const request: RequestPasswordResetRequest = {
        usuario_correo: email,
      };

      const response: string = await invoke("request_password_reset", {
        request,
      });
      setMessage(response);
      setStep("verify");
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const handleVerifyCode = async () => {
    if (!resetCode || resetCode.length !== 6) {
      setError("Por favor ingresa un código de 6 dígitos");
      return;
    }

    setLoading(true);
    setError("");

    try {
      const isValid: boolean = await invoke("verify_reset_code", { resetCode });
      if (isValid) {
        setStep("reset");
        setMessage("Código verificado correctamente");
      } else {
        setError("Código inválido o expirado");
      }
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const handleResetPassword = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError("");

    if (newPassword !== confirmPassword) {
      setError("Las contraseñas no coinciden");
      setLoading(false);
      return;
    }

    if (newPassword.length < 6) {
      setError("La contraseña debe tener al menos 6 caracteres");
      setLoading(false);
      return;
    }

    try {
      const request: ResetPasswordRequest = {
        reset_code: resetCode,
        nueva_contrasena: newPassword,
      };
      const response: string = await invoke("reset_password_with_code", {
        request,
      });
      setMessage(response);
      setStep("success");

      // Redirigir al login después de 3 segundos
      setTimeout(() => {
        setCurrentView("login");
      }, 3000);
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const resetForm = () => {
    setStep("request");
    setEmail("");
    setResetCode("");
    setNewPassword("");
    setConfirmPassword("");
    setMessage("");
    setError("");
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>Recuperar Contraseña</CardTitle>{" "}
          <CardDescription>
            {step === "request" &&
              "Ingresa tu correo electrónico para recibir un código de recuperación"}
            {step === "verify" &&
              "Ingresa el código de 6 dígitos que enviamos a tu correo"}
            {step === "reset" && "Ingresa tu nueva contraseña"}
            {step === "success" && "¡Contraseña cambiada exitosamente!"}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {message && (
            <Alert>
              <AlertDescription>{message}</AlertDescription>
            </Alert>
          )}

          {error && (
            <Alert variant="destructive">
              <AlertDescription>{error}</AlertDescription>
            </Alert>
          )}

          {step === "request" && (
            <form onSubmit={handleRequestReset} className="space-y-4">
              <div>
                <Label htmlFor="email">Correo Electrónico</Label>
                <Input
                  id="email"
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required
                  placeholder="tu@correo.com"
                />
              </div>
              <Button type="submit" className="w-full" disabled={loading}>
                {loading ? "Enviando..." : "Enviar Código"}
              </Button>
            </form>
          )}

          {step === "verify" && (
            <div className="space-y-4">
              <div>
                <Label htmlFor="code">Código de Verificación</Label>
                <Input
                  id="code"
                  type="text"
                  value={resetCode}
                  onChange={(e) =>
                    setResetCode(e.target.value.replace(/\D/g, "").slice(0, 6))
                  }
                  placeholder="123456"
                  maxLength={6}
                  className="text-center text-lg tracking-widest"
                />
              </div>
              <Button
                onClick={handleVerifyCode}
                className="w-full"
                disabled={loading}
              >
                {loading ? "Verificando..." : "Verificar Código"}
              </Button>
              <Button variant="outline" onClick={resetForm} className="w-full">
                Volver al inicio
              </Button>
            </div>
          )}

          {step === "reset" && (
            <form onSubmit={handleResetPassword} className="space-y-4">
              <div>
                <Label htmlFor="newPassword">Nueva Contraseña</Label>
                <Input
                  id="newPassword"
                  type="password"
                  value={newPassword}
                  onChange={(e) => setNewPassword(e.target.value)}
                  required
                  minLength={6}
                  placeholder="Mínimo 6 caracteres"
                />
              </div>
              <div>
                <Label htmlFor="confirmPassword">Confirmar Contraseña</Label>
                <Input
                  id="confirmPassword"
                  type="password"
                  value={confirmPassword}
                  onChange={(e) => setConfirmPassword(e.target.value)}
                  required
                  minLength={6}
                  placeholder="Repite la contraseña"
                />
              </div>
              <Button type="submit" className="w-full" disabled={loading}>
                {loading ? "Cambiando..." : "Cambiar Contraseña"}
              </Button>{" "}
              <Button variant="outline" onClick={resetForm} className="w-full">
                Cancelar
              </Button>
            </form>
          )}

          {step === "success" && (
            <div className="space-y-4 text-center">
              <div className="text-green-600 text-lg font-medium">
                ✓ Contraseña cambiada exitosamente
              </div>
              <p className="text-gray-600">
                Serás redirigido al inicio de sesión en unos segundos...
              </p>
              <Button
                onClick={() => setCurrentView("login")}
                className="w-full"
              >
                Ir al inicio de sesión ahora
              </Button>
            </div>
          )}

          {/* Back to Login button - visible for all steps except success */}
          {step !== "success" && (
            <div className="pt-4 border-t">
              <button
                type="button"
                onClick={() => setCurrentView("login")}
                className="w-full text-sm text-blue-600 hover:text-blue-800 hover:underline focus:outline-none focus:underline"
              >
                ← Volver al inicio de sesión
              </button>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
