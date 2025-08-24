import React, { useState } from "react";
import { useAuth } from "@/contexts/AuthContext";
import { useView } from "@/contexts/ViewContext";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";
import { LoginError } from "@/components/ui/login-error";
import { InitSetup } from "@/components/InitSetup";

export function LoginView() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loginAttempts, setLoginAttempts] = useState(0);
  const [fieldErrors, setFieldErrors] = useState({ email: "", password: "" });
  const [isLoginLoading, setIsLoginLoading] = useState(false);
  const { login } = useAuth();
  const { setCurrentView } = useView();

  const validateFields = () => {
    const errors = { email: "", password: "" };
    let hasErrors = false;

    if (!email.trim()) {
      errors.email = "El correo electrónico es requerido";
      hasErrors = true;
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
      errors.email = "Ingresa un correo electrónico válido";
      hasErrors = true;
    }

    if (!password.trim()) {
      errors.password = "La contraseña es requerida";
      hasErrors = true;
    } else if (password.length < 3) {
      errors.password = "La contraseña debe tener al menos 3 caracteres";
      hasErrors = true;
    }

    setFieldErrors(errors);
    return !hasErrors;
  };
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setFieldErrors({ email: "", password: "" });
    setIsLoginLoading(true);

    if (!validateFields()) {
      setIsLoginLoading(false);
      return;
    }

    try {
      const result = await login(email, password);

      if (!result.success) {
        setLoginAttempts((prev) => prev + 1);
        // Para seguridad, usamos un mensaje genérico para todos los tipos de error
        setError("INVALID_CREDENTIALS");

        // Limpiar la contraseña después de un fallo para seguridad
        setPassword("");
      }
    } catch (err) {
      setError("NETWORK_ERROR");
    } finally {
      setIsLoginLoading(false);
    }
  };
  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="w-full max-w-md space-y-4">
        <InitSetup />
        <Card>
          <CardHeader className="space-y-1">
            <CardTitle className="text-2xl font-bold text-center">
              Iniciar Sesión
            </CardTitle>
            <CardDescription className="text-center">
              Ingresa tus credenciales para acceder al sistema
            </CardDescription>
          </CardHeader>{" "}
          <CardContent>
            <form onSubmit={handleSubmit} className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="email">Email</Label>
                <Input
                  id="email"
                  type="email"
                  placeholder="tu@email.com"
                  value={email}
                  onChange={(e) => {
                    setEmail(e.target.value);
                    if (fieldErrors.email) {
                      setFieldErrors((prev) => ({ ...prev, email: "" }));
                    }
                  }}
                  disabled={isLoginLoading}
                  autoComplete="email"
                  className={
                    fieldErrors.email
                      ? "border-red-500 focus:border-red-500"
                      : ""
                  }
                />
                {fieldErrors.email && (
                  <p className="text-sm text-red-600 mt-1">
                    {fieldErrors.email}
                  </p>
                )}
              </div>
              <div className="space-y-2">
                <Label htmlFor="password">Contraseña</Label>
                <Input
                  id="password"
                  type="password"
                  placeholder="••••••••"
                  value={password}
                  onChange={(e) => {
                    setPassword(e.target.value);
                    if (fieldErrors.password) {
                      setFieldErrors((prev) => ({ ...prev, password: "" }));
                    }
                  }}
                  disabled={isLoginLoading}
                  autoComplete="current-password"
                  className={
                    fieldErrors.password
                      ? "border-red-500 focus:border-red-500"
                      : ""
                  }
                />
                {fieldErrors.password && (
                  <p className="text-sm text-red-600 mt-1">
                    {fieldErrors.password}
                  </p>
                )}
              </div>{" "}
              {error && <LoginError error={error} className="mt-3" />}
              {loginAttempts >= 3 && (
                <div className="text-center">
                  <p className="text-sm text-orange-600 mb-2">
                    Has intentado iniciar sesión {loginAttempts} veces sin
                    éxito.
                  </p>
                  <button
                    type="button"
                    onClick={() => setCurrentView("password-reset")}
                    className="text-sm text-blue-600 hover:text-blue-800 hover:underline focus:outline-none focus:underline"
                  >
                    ¿Necesitas recuperar tu contraseña?
                  </button>
                </div>
              )}{" "}
              <Button
                type="submit"
                className="w-full"
                disabled={isLoginLoading}
              >
                {isLoginLoading ? "Iniciando sesión..." : "Iniciar Sesión"}
              </Button>
              <div className="text-center">
                <button
                  type="button"
                  onClick={() => setCurrentView("password-reset")}
                  className="text-sm text-blue-600 hover:text-blue-800 hover:underline focus:outline-none focus:underline"
                >
                  ¿Olvidaste la contraseña?
                </button>
              </div>
            </form>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
