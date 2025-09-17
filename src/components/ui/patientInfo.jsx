import { ArrowDown, ArrowUp, FileText } from "lucide-react";
import { Button } from "./button";
import { Separator } from "./separator";
import { useEffect, useState, useRef } from "react";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import Visita from "./visita";

function PatientInfo({ selected }) {
  const [showing, setShowing] = useState(0);
  const refs = useRef([]);

  useEffect(() => {
    refs.current[showing]?.scrollIntoView({
      behavior: "smooth",
      scrollMarginTop: "20px",
    });
  }, [showing]);

  return (
    <Dialog>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Agregar Visita</DialogTitle>
        </DialogHeader>
        <Separator />
        <Visita />
        <Separator />
        <div className="flex gap-2 justify-self-end">
          <Button asChild variant={"outline"}>
            <DialogClose>Cancelar</DialogClose>
          </Button>
          <Button>Agregar</Button>
        </div>
      </DialogContent>
      <div className=" flex flex-col h-full">
        {/* Info */}
        <div className="flex items-center justify-between">
          <span className="">
            {selected?.name} {selected?.surname} / {selected?.medicare_number} /{" "}
            {selected?.national_id}
          </span>

          <Button className="cursor-pointer" onClick={() => console.log(refs)}>
            Editar
          </Button>
        </div>

        <Separator className="my-5" />
        {/* Visits */}
        <div className="flex items-center justify-between mb-5">
          <span className="flex items-center gap-2 text-xl">
            <FileText /> Historial de Visitas
          </span>

          <Button asChild>
            <DialogTrigger>Agregar Visita</DialogTrigger>
          </Button>
        </div>

        {/* scroll */}
        <div className="h-full grid grid-cols-[1fr_50px] min-h-0 relative rounded-md border">
          <div className="flex flex-col px-20 overflow-auto">
            {new Array(10).fill("").map((_, i) => (
              <>
                <Visita
                  readOnly
                  className="py-5"
                  visita={{
                    fecha: new Date("05-12-2025"),
                    motivo: "test",
                    diagnostico: "Test\nTest",
                  }}
                  ref={(el) => refs.current.push(el)}
                />
                {i < 9 ? <Separator /> : null}
              </>
            ))}
          </div>

          <div className="self-center flex flex-col gap-1">
            <Button
              className="h-[75px]"
              onClick={() =>
                setShowing((prev) => {
                  if (prev - 1 < 0) {
                    return 9;
                  } else {
                    return prev - 1;
                  }
                })
              }
            >
              <ArrowUp />
            </Button>
            <Button
              className="h-[75px]"
              onClick={() =>
                setShowing((prev) => {
                  if (prev + 1 > 9) {
                    return 0;
                  } else {
                    return prev + 1;
                  }
                })
              }
            >
              <ArrowDown />
            </Button>
          </div>
        </div>
      </div>
    </Dialog>
  );
}

export default PatientInfo;
