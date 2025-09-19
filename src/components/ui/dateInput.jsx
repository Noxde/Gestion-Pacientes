"use client";

import * as React from "react";
import { CalendarIcon } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Calendar } from "@/components/ui/calendar";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";

function formatDate(date) {
  if (!date) {
    return "";
  }

  return date.toLocaleDateString("es-AR", {
    day: "2-digit",
    month: "long",
  });
}

function isValidDate(date) {
  if (!date) {
    return false;
  }
  return !isNaN(date.getTime());
}

export function Calendar28({ label, readOnly, dateValue, onChange }) {
  const [open, setOpen] = React.useState(false);
  const [date, setDate] = React.useState(dateValue ?? new Date());
  const [month, setMonth] = React.useState(date);
  const [value, setValue] = React.useState(formatDate(date));

  return (
    <div className="flex-1 flex flex-col gap-1">
      <Label htmlFor="date" className="text-md">
        {label}
      </Label>
      <Popover open={open} onOpenChange={!readOnly ? setOpen : null}>
        <PopoverTrigger asChild>
          <div className="relative flex gap-2">
            <Input
              id="date"
              value={value}
              readOnly={readOnly}
              placeholder="Junio 01, 2025"
              className="bg-background pr-10"
              // onChange={(e) => {
              //   const date = new Date(e.target.value);
              //   setValue(e.target.value);
              //   if (isValidDate(date)) {
              //     setDate(date);
              //     setMonth(date);
              //   }
              // }}
              onKeyDown={(e) => {
                if (e.key === "ArrowDown") {
                  e.preventDefault();
                  setOpen(true);
                }
              }}
            />
            <Button
              id="date-picker"
              disabled={readOnly}
              variant="ghost"
              className="absolute top-1/2 right-2 size-6 -translate-y-1/2"
              tabIndex={-1}
              type="button"
            >
              <CalendarIcon className="size-3.5" />
              <span className="sr-only">Select date</span>
            </Button>
          </div>
        </PopoverTrigger>
        <PopoverContent
          className="w-auto overflow-hidden p-0"
          align="end"
          alignOffset={-8}
          sideOffset={10}
        >
          <Calendar
            mode="single"
            selected={date}
            captionLayout="dropdown"
            month={month}
            onMonthChange={setMonth}
            onSelect={(date) => {
              setDate(date);
              setValue(formatDate(date));
              if (typeof onChange == "function") {
                onChange(date);
              }
              setOpen(false);
            }}
          />
        </PopoverContent>
      </Popover>
    </div>
  );
}
