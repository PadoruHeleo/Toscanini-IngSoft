"use client";

import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useAuth } from "@/contexts/AuthContext";
import { ViewTitle } from "@/components/ViewTitle";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { IconCheck, IconX, IconEye, IconEyeOff } from "@tabler/icons-react";

interface ChangePasswordRequest {
  current_password: string;
  new_password: string;
}

interface ChangeEmailRequest {
  new_email: string;
  password: string;
}

export function SettingsView() {
  const { user, validateSession } = useAuth();

  // Estados para cambio de contraseña
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [showCurrentPassword, setShowCurrentPassword] = useState(false);
  const [showNewPassword, setShowNewPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  const [isChangingPassword, setIsChangingPassword] = useState(false);
  const [passwordSuccess, setPasswordSuccess] = useState("");
  const [passwordError, setPasswordError] = useState("");

  // Estados para cambio de email
  const [newEmail, setNewEmail] = useState("");
  const [emailPassword, setEmailPassword] = useState("");
  const [showEmailPassword, setShowEmailPassword] = useState(false);
  const [isChangingEmail, setIsChangingEmail] = useState(false);
  const [emailSuccess, setEmailSuccess] = useState("");
  const [emailError, setEmailError] = useState("");

  const handleChangePassword = async (e: React.FormEvent) => {
    e.preventDefault();
    setPasswordError("");
    setPasswordSuccess("");

    // Validaciones
    if (newPassword.length < 6) {
      setPasswordError("La nueva contraseña debe tener al menos 6 caracteres");
      return;
    }

    if (newPassword !== confirmPassword) {
      setPasswordError("Las contraseñas no coinciden");
      return;
    }

    if (currentPassword === newPassword) {
      setPasswordError("La nueva contraseña debe ser diferente a la actual");
      return;
    }

    setIsChangingPassword(true);

    try {
      const request: ChangePasswordRequest = {
        current_password: currentPassword,
        new_password: newPassword,
      };

      const result = await invoke<boolean>("change_user_password", {
        usuarioId: user!.usuario_id,
        request,
      });

      if (result) {
        setPasswordSuccess("Contraseña cambiada exitosamente");
        setCurrentPassword("");
        setNewPassword("");
        setConfirmPassword("");
      } else {
        setPasswordError("No se pudo cambiar la contraseña");
      }
    } catch (error) {
      const errorMessage = String(error);
      if (errorMessage.includes("Contraseña actual incorrecta")) {
        setPasswordError("La contraseña actual es incorrecta");
      } else {
        setPasswordError("Error al cambiar la contraseña: " + errorMessage);
      }
    } finally {
      setIsChangingPassword(false);
    }
  };

  const handleChangeEmail = async (e: React.FormEvent) => {
    e.preventDefault();
    setEmailError("");
    setEmailSuccess("");

    // Validaciones
    if (!newEmail.includes("@") || !newEmail.includes(".")) {
      setEmailError("Ingresa un email válido");
      return;
    }

    if (newEmail === user?.usuario_correo) {
      setEmailError("El nuevo email debe ser diferente al actual");
      return;
    }

    setIsChangingEmail(true);

    try {
      const request: ChangeEmailRequest = {
        new_email: newEmail,
        password: emailPassword,
      };

      const result = await invoke<any>("change_user_email", {
        usuarioId: user!.usuario_id,
        request,
      });

      if (result) {
        setEmailSuccess(
          "Email cambiado exitosamente. Por favor, inicia sesión nuevamente."
        );
        setNewEmail("");
        setEmailPassword("");

        // Revalidar sesión para obtener los datos actualizados
        await validateSession();
      }
    } catch (error) {
      const errorMessage = String(error);
      if (errorMessage.includes("Contraseña incorrecta")) {
        setEmailError("La contraseña es incorrecta");
      } else if (errorMessage.includes("ya está en uso")) {
        setEmailError("Este email ya está en uso por otro usuario");
      } else {
        setEmailError("Error al cambiar el email: " + errorMessage);
      }
    } finally {
      setIsChangingEmail(false);
    }
  };
  if (!user) {
    return (
      <div className="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
        <ViewTitle />
        <div className="flex justify-center">
          <div className="border border-dashed rounded-lg p-6 text-center max-w-md">
            <p className="text-muted-foreground">
              Debes iniciar sesión para ver la configuración
            </p>
          </div>
        </div>
      </div>
    );
  }
  return (
    <div className="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
      <ViewTitle />

      <div className="flex justify-center">
        <div className="grid gap-6 max-w-2xl w-full">
          {/* Información del usuario */}
          <Card>
            <CardHeader>
              <CardTitle>Información del Usuario</CardTitle>
              <CardDescription>Información básica de tu cuenta</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label className="text-sm font-medium">Nombre</Label>
                  <p className="text-sm text-muted-foreground">
                    {user.usuario_nombre || "Sin nombre"}
                  </p>
                </div>
                <div>
                  <Label className="text-sm font-medium">RUT</Label>
                  <p className="text-sm text-muted-foreground">
                    {user.usuario_rut || "Sin RUT"}
                  </p>
                </div>
                <div>
                  <Label className="text-sm font-medium">Email actual</Label>
                  <p className="text-sm text-muted-foreground">
                    {user.usuario_correo || "Sin email"}
                  </p>
                </div>
                <div>
                  <Label className="text-sm font-medium">Rol</Label>
                  <p className="text-sm text-muted-foreground">
                    {user.usuario_rol || "Sin rol"}
                  </p>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Cambiar contraseña */}
          <Card>
            <CardHeader>
              <CardTitle>Cambiar Contraseña</CardTitle>
              <CardDescription>
                Cambia tu contraseña actual por una nueva
              </CardDescription>
            </CardHeader>
            <CardContent>
              <form onSubmit={handleChangePassword} className="space-y-4">
                {passwordSuccess && (
                  <Alert className="border-green-200 bg-green-50">
                    <IconCheck className="h-4 w-4 text-green-600" />
                    <AlertDescription className="text-green-800">
                      {passwordSuccess}
                    </AlertDescription>
                  </Alert>
                )}

                {passwordError && (
                  <Alert className="border-red-200 bg-red-50">
                    <IconX className="h-4 w-4 text-red-600" />
                    <AlertDescription className="text-red-800">
                      {passwordError}
                    </AlertDescription>
                  </Alert>
                )}

                <div className="space-y-2">
                  <Label htmlFor="current-password">Contraseña actual</Label>
                  <div className="relative">
                    <Input
                      id="current-password"
                      type={showCurrentPassword ? "text" : "password"}
                      value={currentPassword}
                      onChange={(e) => setCurrentPassword(e.target.value)}
                      required
                      disabled={isChangingPassword}
                      className="pr-10"
                    />
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      className="absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent"
                      onClick={() =>
                        setShowCurrentPassword(!showCurrentPassword)
                      }
                    >
                      {showCurrentPassword ? (
                        <IconEyeOff className="h-4 w-4" />
                      ) : (
                        <IconEye className="h-4 w-4" />
                      )}
                    </Button>
                  </div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="new-password">Nueva contraseña</Label>
                  <div className="relative">
                    <Input
                      id="new-password"
                      type={showNewPassword ? "text" : "password"}
                      value={newPassword}
                      onChange={(e) => setNewPassword(e.target.value)}
                      required
                      disabled={isChangingPassword}
                      className="pr-10"
                      minLength={6}
                    />
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      className="absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent"
                      onClick={() => setShowNewPassword(!showNewPassword)}
                    >
                      {showNewPassword ? (
                        <IconEyeOff className="h-4 w-4" />
                      ) : (
                        <IconEye className="h-4 w-4" />
                      )}
                    </Button>
                  </div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="confirm-password">
                    Confirmar nueva contraseña
                  </Label>
                  <div className="relative">
                    <Input
                      id="confirm-password"
                      type={showConfirmPassword ? "text" : "password"}
                      value={confirmPassword}
                      onChange={(e) => setConfirmPassword(e.target.value)}
                      required
                      disabled={isChangingPassword}
                      className="pr-10"
                      minLength={6}
                    />
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      className="absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent"
                      onClick={() =>
                        setShowConfirmPassword(!showConfirmPassword)
                      }
                    >
                      {showConfirmPassword ? (
                        <IconEyeOff className="h-4 w-4" />
                      ) : (
                        <IconEye className="h-4 w-4" />
                      )}
                    </Button>
                  </div>
                </div>

                <Button
                  type="submit"
                  disabled={isChangingPassword}
                  className="w-full"
                >
                  {isChangingPassword
                    ? "Cambiando contraseña..."
                    : "Cambiar contraseña"}
                </Button>
              </form>
            </CardContent>
          </Card>

          {/* Cambiar email */}
          <Card>
            <CardHeader>
              <CardTitle>Cambiar Email</CardTitle>
              <CardDescription>
                Cambia tu dirección de correo electrónico
              </CardDescription>
            </CardHeader>
            <CardContent>
              <form onSubmit={handleChangeEmail} className="space-y-4">
                {emailSuccess && (
                  <Alert className="border-green-200 bg-green-50">
                    <IconCheck className="h-4 w-4 text-green-600" />
                    <AlertDescription className="text-green-800">
                      {emailSuccess}
                    </AlertDescription>
                  </Alert>
                )}

                {emailError && (
                  <Alert className="border-red-200 bg-red-50">
                    <IconX className="h-4 w-4 text-red-600" />
                    <AlertDescription className="text-red-800">
                      {emailError}
                    </AlertDescription>
                  </Alert>
                )}

                <div className="space-y-2">
                  <Label htmlFor="new-email">Nuevo email</Label>
                  <Input
                    id="new-email"
                    type="email"
                    value={newEmail}
                    onChange={(e) => setNewEmail(e.target.value)}
                    required
                    disabled={isChangingEmail}
                    placeholder="nuevo@email.com"
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="email-password">Confirma tu contraseña</Label>
                  <div className="relative">
                    <Input
                      id="email-password"
                      type={showEmailPassword ? "text" : "password"}
                      value={emailPassword}
                      onChange={(e) => setEmailPassword(e.target.value)}
                      required
                      disabled={isChangingEmail}
                      className="pr-10"
                    />
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      className="absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent"
                      onClick={() => setShowEmailPassword(!showEmailPassword)}
                    >
                      {showEmailPassword ? (
                        <IconEyeOff className="h-4 w-4" />
                      ) : (
                        <IconEye className="h-4 w-4" />
                      )}
                    </Button>
                  </div>
                </div>

                <Button
                  type="submit"
                  disabled={isChangingEmail}
                  className="w-full"
                >
                  {isChangingEmail ? "Cambiando email..." : "Cambiar email"}
                </Button>
              </form>
            </CardContent>{" "}
          </Card>
        </div>
      </div>
    </div>
  );
}
