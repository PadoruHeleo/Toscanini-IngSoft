"use client";

import { DataTable } from "@/components/data-table";
import { ViewTitle } from "@/components/ViewTitle";
import data from "@/consts/data.json";

export function DashboardView() {
  return (
    <div className="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
      <ViewTitle />
      <DataTable data={data}/>
    </div>
  );
}
