import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/shadcn/dialog";

function Image({ src, name }) {
  return (
    <Dialog>
      <DialogContent className="max-h-[calc(100svh-20px)] w-fit flex flex-col items-center overflow-hidden">
        <DialogTitle className="self-start">Imagen</DialogTitle>
        <img className="inline-block max-h-[calc(100svh-150px)]" src={src} />
        <DialogHeader className="self-start font-medium">{name}</DialogHeader>
      </DialogContent>
      <DialogTrigger asChild className="cursor-pointer">
        <div className="rounded-2xl overflow-hidden relative">
          {/* Name */}
          <div className="absolute z-50 bg-linear-to-t from-[#000000e8] to-transparent p-2 h-full w-full flex items-end">
            <span className="text-white font-semibold">{name}</span>
          </div>
          <img
            className="w-full h-full inline-block object-cover object-center"
            src={src}
          />
        </div>
      </DialogTrigger>
    </Dialog>
  );
}

export default Image;
