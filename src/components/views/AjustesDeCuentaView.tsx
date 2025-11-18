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
import { Separator } from "@/components/ui/separator";
import {
  IconEye,
  IconEyeOff,
  IconMail,
  IconPhone,
  IconUser,
  IconLock,
} from "@tabler/icons-react";

interface ChangeEmailRequest {
  new_email: string;
  password: string;
}

interface ChangePhoneRequest {
  new_phone: string;
  password: string;
}

interface ChangePasswordRequest {
  current_password: string;
  new_password: string;
}

export function AjustesDeCuentaView() {
  const { user, validateSession } = useAuth();

  // Estados para cambios
  const [newEmail, setNewEmail] = useState("");
  const [newPhone, setNewPhone] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);

  // Estados para cambio de contraseña
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [showNewPassword, setShowNewPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);

  // Estados de carga y mensajes
  const [isChangingEmail, setIsChangingEmail] = useState(false);
  const [isChangingPhone, setIsChangingPhone] = useState(false);
  const [isChangingPassword, setIsChangingPassword] = useState(false);
  const [successMessage, setSuccessMessage] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  const clearMessages = () => {
    setSuccessMessage("");
    setErrorMessage("");
  };

  const handleChangeEmail = async (e: React.FormEvent) => {
    e.preventDefault();
    clearMessages();

    // Validaciones
    if (!newEmail.includes("@") || !newEmail.includes(".")) {
      setErrorMessage("Ingresa un email válido");
      return;
    }

    if (newEmail === user?.usuario_correo) {
      setErrorMessage("El nuevo email debe ser diferente al actual");
      return;
    }

    if (!password) {
      setErrorMessage("Debes ingresar tu contraseña para confirmar el cambio");
      return;
    }

    setIsChangingEmail(true);
    try {
      const emailEnUso = await invoke<boolean>("verify_email_in_use", {
        correo: newEmail,
      });
      if (emailEnUso) {
        setErrorMessage("Este email ya está en uso por otro usuario");
        setIsChangingEmail(false);
        return;
      }

      const request: ChangeEmailRequest = {
        new_email: newEmail,
        password: password,
      };

      const result = await invoke<any>("change_user_email", {
        usuarioId: user!.usuario_id,
        request,
      });

      if (result) {
        setSuccessMessage(
          "Email cambiado exitosamente. Por favor, inicia sesión nuevamente."
        );
        setNewEmail("");
        setPassword("");
        await validateSession();
      }
    } catch (error) {
      const errorMessage = String(error);
      if (errorMessage.includes("Contraseña incorrecta")) {
        setErrorMessage("La contraseña es incorrecta");
      } else if (errorMessage.includes("ya está en uso")) {
        setErrorMessage("Este email ya está en uso por otro usuario");
      } else {
        setErrorMessage("Error al cambiar el email: " + errorMessage);
      }
    } finally {
      setIsChangingEmail(false);
    }
  };

  const handleChangePhone = async (e: React.FormEvent) => {
    e.preventDefault();
    clearMessages();

    if (!password) {
      setErrorMessage("Debes ingresar tu contraseña para confirmar el cambio");
      return;
    }

    // Validar formato usando Tauri verify_phone
    try {
      const isValid = await invoke<boolean>("verify_phone", {
        phone: newPhone,
      });
      if (!isValid) {
        setErrorMessage(
          "El teléfono debe iniciar con '+' y tener solo números, máximo 12 caracteres."
        );
        return;
      }
    } catch (err) {
      setErrorMessage("Error al validar el formato del teléfono");
      return;
    }

    if (newPhone === user?.usuario_telefono) {
      setErrorMessage("El nuevo teléfono debe ser diferente al actual");
      return;
    }

    setIsChangingPhone(true);

    try {
      const request: ChangePhoneRequest = {
        new_phone: newPhone,
        password: password,
      };

      const result = await invoke<any>("change_user_phone", {
        usuarioId: user!.usuario_id,
        request,
      });

      if (result) {
        setSuccessMessage("Teléfono cambiado exitosamente");
        setNewPhone("");
        setPassword("");
        await validateSession();
      }
    } catch (error) {
      const errorMsg = String(error);
      if (errorMsg.includes("Contraseña incorrecta")) {
        setErrorMessage("La contraseña es incorrecta");
      } else {
        setErrorMessage("Error al cambiar el teléfono: " + errorMsg);
      }
    } finally {
      setIsChangingPhone(false);
    }
  };

  const handleChangePassword = async (e: React.FormEvent) => {
    e.preventDefault();
    clearMessages();

    // Validaciones
    if (!password) {
      setErrorMessage("Debes ingresar tu contraseña actual");
      return;
    }

    if (newPassword.length < 6) {
      setErrorMessage("La nueva contraseña debe tener al menos 6 caracteres");
      return;
    }

    if (newPassword !== confirmPassword) {
      setErrorMessage("Las contraseñas nuevas no coinciden");
      return;
    }

    if (password === newPassword) {
      setErrorMessage("La nueva contraseña debe ser diferente a la actual");
      return;
    }

    setIsChangingPassword(true);

    try {
      const request: ChangePasswordRequest = {
        current_password: password,
        new_password: newPassword,
      };

      const result = await invoke<boolean>("change_user_password", {
        usuarioId: user!.usuario_id,
        request,
      });

      if (result) {
        setSuccessMessage("Contraseña cambiada exitosamente");
        setPassword("");
        setNewPassword("");
        setConfirmPassword("");
      }
    } catch (error) {
      const errorMsg = String(error);
      if (
        errorMsg.includes("Contraseña actual incorrecta") ||
        errorMsg.includes("incorrecta")
      ) {
        setErrorMessage("La contraseña actual es incorrecta");
      } else {
        setErrorMessage("Error al cambiar la contraseña: " + errorMsg);
      }
    } finally {
      setIsChangingPassword(false);
    }
  };

  if (!user) {
    return (
      <div className="flex flex-col gap-4 py-4 px-6 md:gap-6 md:py-6 md:px-12">
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
    <div className="flex flex-col gap-4 py-2 px-6 md:gap-4 md:py-3 md:px-8">
      <ViewTitle />

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Información del usuario */}
        <Card className="max-h-[280px]">
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-xl font-bold">
              <IconUser className="h-5 w-5" />
              Información de la Cuenta
            </CardTitle>
            <CardDescription className="text-xs mt-0.5">
              Información básica de tu cuenta de usuario
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-2.5">
              <div>
                <Label className="text-xs font-medium text-muted-foreground">
                  Nombre
                </Label>
                <p className="text-sm font-medium mt-0.5">
                  {user.usuario_nombre || "Sin nombre"}
                </p>
              </div>
              <div>
                <Label className="text-xs font-medium text-muted-foreground">
                  RUT
                </Label>
                <p className="text-sm font-medium mt-0.5">
                  {user.usuario_rut || "Sin RUT"}
                </p>
              </div>
              <div>
                <Label className="text-xs font-medium text-muted-foreground">
                  Rol
                </Label>
                <p className="text-sm font-medium mt-0.5">
                  {user.usuario_rol || "Sin rol"}
                </p>
              </div>
              <div>
                <Label className="text-xs font-medium text-muted-foreground">
                  Email
                </Label>
                <p className="text-sm font-medium mt-0.5">
                  {user.usuario_correo || "Sin email"}
                </p>
              </div>
              <div className="md:col-span-2">
                <Label className="text-xs font-medium text-muted-foreground">
                  Teléfono
                </Label>
                <p className="text-sm font-medium mt-0.5">
                  {user.usuario_telefono || "Sin teléfono"}
                </p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Configuración de cuenta */}
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg">Configuración de Cuenta</CardTitle>
            <CardDescription className="text-xs">
              Actualiza tu información de contacto. Se requiere confirmar con tu
              contraseña para realizar cambios.
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* Mensajes de éxito/error */}
            {successMessage && (
              <Alert className="border-green-200 bg-green-50 py-2">
                <AlertDescription className="text-sm text-green-800">
                  {successMessage}
                </AlertDescription>
              </Alert>
            )}

            {errorMessage && (
              <Alert className="border-red-200 bg-red-50 py-2">
                <AlertDescription className="text-sm text-red-800">
                  {errorMessage}
                </AlertDescription>
              </Alert>
            )}

            {/* Campo de contraseña compartido */}
            <div className="space-y-1">
              <Label htmlFor="password" className="text-sm">
                Contraseña actual
              </Label>
              <div className="relative">
                <Input
                  id="password"
                  type={showPassword ? "text" : "password"}
                  value={password}
                  onChange={(e) => {
                    setPassword(e.target.value);
                    clearMessages();
                  }}
                  disabled={
                    isChangingEmail || isChangingPhone || isChangingPassword
                  }
                  placeholder="Ingresa tu contraseña para confirmar cambios"
                  className="pr-10"
                />
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  className="absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent"
                  onClick={() => setShowPassword(!showPassword)}
                  disabled={
                    isChangingEmail || isChangingPhone || isChangingPassword
                  }
                >
                  {showPassword ? (
                    <IconEyeOff className="h-4 w-4" />
                  ) : (
                    <IconEye className="h-4 w-4" />
                  )}
                </Button>
              </div>
            </div>

            <Separator />

            {/* Cambiar Email */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <IconMail className="h-4 w-4 text-muted-foreground" />
                <Label className="text-base font-semibold">Cambiar Email</Label>
              </div>
              <form onSubmit={handleChangeEmail} className="space-y-2">
                <div className="space-y-1">
                  <Label htmlFor="new-email" className="text-sm">
                    Nuevo email
                  </Label>
                  <Input
                    id="new-email"
                    type="email"
                    value={newEmail}
                    onChange={(e) => {
                      setNewEmail(e.target.value);
                      clearMessages();
                    }}
                    required
                    disabled={
                      isChangingEmail || isChangingPhone || isChangingPassword
                    }
                    placeholder="nuevo@email.com"
                  />
                </div>
                <Button
                  type="submit"
                  disabled={
                    isChangingEmail || isChangingPhone || !newEmail || !password
                  }
                  className="w-full md:w-auto"
                >
                  {isChangingEmail ? "Cambiando..." : "Actualizar Email"}
                </Button>
              </form>
            </div>

            <Separator />

            {/* Cambiar Teléfono */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <IconPhone className="h-4 w-4 text-muted-foreground" />
                <Label className="text-base font-semibold">
                  Cambiar Teléfono
                </Label>
              </div>
              <form onSubmit={handleChangePhone} className="space-y-2">
                <div className="space-y-1">
                  <Label htmlFor="new-phone" className="text-sm">
                    Nuevo teléfono
                  </Label>
                  <Input
                    id="new-phone"
                    type="tel"
                    value={newPhone}
                    onChange={(e) => {
                      setNewPhone(e.target.value);
                      clearMessages();
                    }}
                    required
                    disabled={
                      isChangingEmail || isChangingPhone || isChangingPassword
                    }
                    placeholder="+56912345678"
                    minLength={12}
                    maxLength={12}
                  />
                </div>
                <Button
                  type="submit"
                  disabled={
                    isChangingEmail ||
                    isChangingPhone ||
                    isChangingPassword ||
                    !newPhone ||
                    !password
                  }
                  className="w-full md:w-auto"
                >
                  {isChangingPhone ? "Cambiando..." : "Actualizar Teléfono"}
                </Button>
              </form>
            </div>

            <Separator />

            {/* Cambiar Contraseña */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <IconLock className="h-4 w-4 text-muted-foreground" />
                <Label className="text-base font-semibold">
                  Cambiar Contraseña
                </Label>
              </div>
              <form onSubmit={handleChangePassword} className="space-y-2">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                  <div className="space-y-1">
                    <Label htmlFor="new-password" className="text-sm">
                      Nueva contraseña
                    </Label>
                    <div className="relative">
                      <Input
                        id="new-password"
                        type={showNewPassword ? "text" : "password"}
                        value={newPassword}
                        onChange={(e) => {
                          setNewPassword(e.target.value);
                          clearMessages();
                        }}
                        required
                        disabled={
                          isChangingEmail ||
                          isChangingPhone ||
                          isChangingPassword
                        }
                        placeholder="Mín. 6 caracteres"
                        minLength={6}
                        className="pr-10"
                      />
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        className="absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent"
                        onClick={() => setShowNewPassword(!showNewPassword)}
                        disabled={
                          isChangingEmail ||
                          isChangingPhone ||
                          isChangingPassword
                        }
                      >
                        {showNewPassword ? (
                          <IconEyeOff className="h-4 w-4" />
                        ) : (
                          <IconEye className="h-4 w-4" />
                        )}
                      </Button>
                    </div>
                  </div>
                  <div className="space-y-1">
                    <Label htmlFor="confirm-password" className="text-sm">
                      Confirmar nueva contraseña
                    </Label>
                    <div className="relative">
                      <Input
                        id="confirm-password"
                        type={showConfirmPassword ? "text" : "password"}
                        value={confirmPassword}
                        onChange={(e) => {
                          setConfirmPassword(e.target.value);
                          clearMessages();
                        }}
                        required
                        disabled={
                          isChangingEmail ||
                          isChangingPhone ||
                          isChangingPassword
                        }
                        placeholder="Confirma tu nueva contraseña"
                        className="pr-10"
                      />
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        className="absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent"
                        onClick={() =>
                          setShowConfirmPassword(!showConfirmPassword)
                        }
                        disabled={
                          isChangingEmail ||
                          isChangingPhone ||
                          isChangingPassword
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
                </div>
                <Button
                  type="submit"
                  disabled={
                    isChangingEmail ||
                    isChangingPhone ||
                    isChangingPassword ||
                    !password ||
                    !newPassword ||
                    !confirmPassword
                  }
                  className="w-full md:w-auto"
                >
                  {isChangingPassword
                    ? "Cambiando..."
                    : "Actualizar Contraseña"}
                </Button>
              </form>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
