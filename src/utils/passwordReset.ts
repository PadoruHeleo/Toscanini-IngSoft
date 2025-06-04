// Ejemplo de uso de las funciones de recuperación de contraseña desde JavaScript/TypeScript

import { invoke } from "@tauri-apps/api/core";

// Tipos para TypeScript
interface RequestPasswordResetRequest {
  usuario_correo: string;
}

interface ResetPasswordRequest {
  reset_code: string;
  nueva_contrasena: string;
}

// Función para solicitar recuperación de contraseña
export async function requestPasswordReset(email: string): Promise<string> {
  try {
    const request: RequestPasswordResetRequest = {
      usuario_correo: email,
    };

    const response = await invoke<string>("request_password_reset", {
      request,
    });
    return response;
  } catch (error) {
    throw new Error(error as string);
  }
}

// Función para verificar código de recuperación
export async function verifyResetCode(code: string): Promise<boolean> {
  try {
    const isValid = await invoke<boolean>("verify_reset_code", {
      resetCode: code,
    });
    return isValid;
  } catch (error) {
    throw new Error(error as string);
  }
}

// Función para cambiar contraseña con código
export async function resetPasswordWithCode(
  code: string,
  newPassword: string
): Promise<string> {
  try {
    const request: ResetPasswordRequest = {
      reset_code: code,
      nueva_contrasena: newPassword,
    };

    const response = await invoke<string>("reset_password_with_code", {
      request,
    });
    return response;
  } catch (error) {
    throw new Error(error as string);
  }
}

// Función para limpiar códigos expirados (función de administración)
export async function cleanupExpiredResetCodes(): Promise<number> {
  try {
    const deletedCount = await invoke<number>("cleanup_expired_reset_codes");
    return deletedCount;
  } catch (error) {
    throw new Error(error as string);
  }
}

// Ejemplo de uso completo
export async function examplePasswordResetFlow() {
  try {
    // 1. Solicitar código de recuperación
    const email = "usuario@ejemplo.com";
    const message = await requestPasswordReset(email);
    console.log("Código enviado:", message);

    // 2. Verificar código (esto normalmente lo haría el usuario)
    const userCode = "123456"; // Código que el usuario ingresa
    const isValid = await verifyResetCode(userCode);

    if (isValid) {
      // 3. Cambiar contraseña
      const newPassword = "nuevaContrasena123";
      const successMessage = await resetPasswordWithCode(userCode, newPassword);
      console.log("Contraseña cambiada:", successMessage);
    } else {
      console.log("Código inválido");
    }
  } catch (error) {
    console.error("Error en el proceso:", error);
  }
}
