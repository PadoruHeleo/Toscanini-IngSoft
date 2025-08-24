"use client";

import React, { createContext, useContext, useState } from "react";

type ViewContextType = {
  currentView: string;
  setCurrentView: (view: string) => void;
};

const ViewContext = createContext<ViewContextType | null>(null);

export function ViewProvider({ children }: { children: React.ReactNode }) {
  const [currentView, setCurrentView] = useState<string>("inicio");

  return (
    <ViewContext.Provider value={{ currentView, setCurrentView }}>
      {children}
    </ViewContext.Provider>
  );
}

export function useView() {
  const context = useContext(ViewContext);
  if (!context) {
    throw new Error("useView must be used within a ViewProvider");
  }
  return context;
}
