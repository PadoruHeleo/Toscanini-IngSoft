import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useAuth } from "@/contexts/AuthContext";
import { ViewTitle } from "@/components/layout/ViewTitle";
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


interface ChangeEmailRequest {
  new_email: string;
  password: string;
}

export function AjustesDeCuentaView() {
  const { user, validateSession } = useAuth();


  // Estados para cambio de email
  const [newEmail, setNewEmail] = useState("");
  const [emailPassword, setEmailPassword] = useState("");
  const [showEmailPassword, setShowEmailPassword] = useState(false);
  const [isChangingEmail, setIsChangingEmail] = useState(false);
  const [emailSuccess, setEmailSuccess] = useState("");
  const [emailError, setEmailError] = useState("");

  // Estados para cambio de teléfono
  const [newPhone, setNewPhone] = useState("");
  const [phonePassword, setPhonePassword] = useState("");
  const [showPhonePassword, setShowPhonePassword] = useState(false);
  const [isChangingPhone, setIsChangingPhone] = useState(false);
  const [phoneSuccess, setPhoneSuccess] = useState("");
  const [phoneError, setPhoneError] = useState("");


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

    // Validar email en uso usando Tauri
    setIsChangingEmail(true);
    try {
      const emailEnUso = await invoke<boolean>("verify_email_in_use", { correo: newEmail });
      if (emailEnUso) {
        setEmailError("Este email ya está en uso por otro usuario");
        setIsChangingEmail(false);
        return;
      }

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

  const handleChangePhone = async (e: React.FormEvent) => {
    e.preventDefault();
    setPhoneError("");
    setPhoneSuccess("");

    // Validar formato usando Tauri verify_phone
    try {
      const isValid = await invoke<boolean>("verify_phone", { phone: newPhone });
      if (!isValid) {
        setPhoneError("El teléfono debe iniciar con '+' y tener solo números, máximo 12 caracteres.");
        return;
      }
    } catch (err) {
      setPhoneError("Error al validar el formato del teléfono");
      return;
    }

    if (newPhone === user?.usuario_telefono) {
      setPhoneError("El nuevo teléfono debe ser diferente al actual");
      return;
    }

    setIsChangingPhone(true);

    try {
      const result = await invoke<any>("update_usuario", {
        usuarioId: user!.usuario_id,
        request: { usuario_telefono: newPhone },
      });

      if (result) {
        setPhoneSuccess("Teléfono cambiado exitosamente");
        setNewPhone("");
        setPhonePassword("");

        // Revalidar sesión para obtener los datos actualizados
        await validateSession();
      }
    } catch (error) {
      const errorMessage = String(error);
      setPhoneError("Error al cambiar el teléfono: " + errorMessage);
    } finally {
      setIsChangingPhone(false);
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
                <div>
                  <Label className="text-sm font-medium">Teléfono</Label>
                  <p className="text-sm text-muted-foreground">
                    {user.usuario_telefono || "Sin teléfono"}
                  </p>
                </div>
              </div>
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

          {/* Cambiar teléfono */}
          <Card>
            <CardHeader>
              <CardTitle>Cambiar Teléfono</CardTitle>
              <CardDescription>
                Cambia tu número de teléfono asociado a la cuenta
              </CardDescription>
            </CardHeader>
            <CardContent>
              <form onSubmit={handleChangePhone} className="space-y-4">
                {phoneSuccess && (
                  <Alert className="border-green-200 bg-green-50">
                    <IconCheck className="h-4 w-4 text-green-600" />
                    <AlertDescription className="text-green-800">
                      {phoneSuccess}
                    </AlertDescription>
                  </Alert>
                )}

                {phoneError && (
                  <Alert className="border-red-200 bg-red-50">
                    <IconX className="h-4 w-4 text-red-600" />
                    <AlertDescription className="text-red-800">
                      {phoneError}
                    </AlertDescription>
                  </Alert>
                )}

                <div className="space-y-2">
                  <Label htmlFor="new-phone">Nuevo teléfono</Label>
                  <Input
                    id="new-phone"
                    type="tel"
                    value={newPhone}
                    onChange={(e) => setNewPhone(e.target.value)}
                    required
                    disabled={isChangingPhone}
                    placeholder="+56912345678"
                    minLength={12}
                    maxLength={12}
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="phone-password">Confirma tu contraseña</Label>
                  <div className="relative">
                    <Input
                      id="phone-password"
                      type={showPhonePassword ? "text" : "password"}
                      value={phonePassword}
                      onChange={(e) => setPhonePassword(e.target.value)}
                      required
                      disabled={isChangingPhone}
                      className="pr-10"
                    />
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      className="absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent"
                      onClick={() => setShowPhonePassword(!showPhonePassword)}
                    >
                      {showPhonePassword ? (
                        <IconEyeOff className="h-4 w-4" />
                      ) : (
                        <IconEye className="h-4 w-4" />
                      )}
                    </Button>
                  </div>
                </div>

                <Button
                  type="submit"
                  disabled={isChangingPhone}
                  className="w-full"
                >
                  {isChangingPhone ? "Cambiando teléfono..." : "Cambiar teléfono"}
                </Button>
              </form>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
