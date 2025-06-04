import { useState, useCallback } from "react";

export interface ToastType {
  id: string;
  title: string;
  description?: string;
  variant: "default" | "destructive" | "success";
  duration?: number;
}

export function useToast() {
  const [toasts, setToasts] = useState<ToastType[]>([]);

  const showToast = useCallback((toast: Omit<ToastType, "id">) => {
    const id = Math.random().toString(36).substring(2, 9);
    const newToast: ToastType = {
      ...toast,
      id,
      duration: toast.duration || 5000,
    };

    setToasts((prev) => [...prev, newToast]);

    // Auto remove toast after duration
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, newToast.duration);

    return id;
  }, []);

  const removeToast = useCallback((id: string) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  }, []);

  const success = useCallback(
    (title: string, description?: string) => {
      return showToast({ title, description, variant: "success" });
    },
    [showToast]
  );

  const error = useCallback(
    (title: string, description?: string) => {
      return showToast({ title, description, variant: "destructive" });
    },
    [showToast]
  );

  const info = useCallback(
    (title: string, description?: string) => {
      return showToast({ title, description, variant: "default" });
    },
    [showToast]
  );

  return {
    toasts,
    showToast,
    removeToast,
    success,
    error,
    info,
  };
}
