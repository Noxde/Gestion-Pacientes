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
  const [toAdd, setToAdd] = useState({});
  const [visitas, setVisitas] = useState([]);

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
        <Visita onChange={(e) => setToAdd(e)} />
        <Separator />

        <div className="flex gap-2 justify-self-end">
          <Button asChild variant={"outline"}>
            <DialogClose>Cancelar</DialogClose>
          </Button>
          <Button
            asChild
            onClick={() => setVisitas((prev) => [toAdd, ...prev])}
          >
            <DialogClose>Agregar</DialogClose>
          </Button>
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
            <FileText /> Historial de Visitas ({visitas.length})
          </span>

          <Button asChild>
            <DialogTrigger>Agregar Visita</DialogTrigger>
          </Button>
        </div>

        {/* scroll */}
        <div className="h-full grid grid-cols-[1fr_50px] min-h-0 relative rounded-md border">
          <div className="flex flex-col px-20 overflow-auto">
            {visitas.map((x, i, arr) => (
              <>
                <Visita
                  readOnly
                  className="py-5"
                  visita={x}
                  ref={(el) => (refs.current[i] = el)}
                />
                {i < arr.length - 1 ? <Separator /> : null}
              </>
            ))}
          </div>

          <div className="self-center flex flex-col gap-1">
            <Button
              className="h-[75px]"
              onClick={() =>
                setShowing((prev) => {
                  if (prev - 1 < 0) {
                    return visitas.length - 1;
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
                  if (prev + 1 > visitas.length - 1) {
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
